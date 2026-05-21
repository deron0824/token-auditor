use anyhow::Result;
use hyper::{Body, Method, Request, Response, StatusCode};
use serde_json::Value;
use std::time::Instant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{error, info, warn};

use super::forwarder;
use crate::audit::cost::CostCalculator;
use crate::audit::prices::PriceManager;
use crate::audit::token::TokenCalculator;
use crate::config::Config;
use crate::display::onboarding::OnboardingDisplay;
use crate::display::terminal::TerminalDisplay;
use crate::storage::cache::{RequestCache, RequestRecord};

/// 代理处理器（包含所有依赖）
pub struct ProxyHandler {
    pub config: Config,
    pub price_manager: PriceManager,
    pub token_calculator: TokenCalculator,
    pub cost_calculator: CostCalculator,
    pub terminal_display: TerminalDisplay,
    pub onboarding: OnboardingDisplay,
    pub request_cache: RequestCache,
}

impl ProxyHandler {
    /// 创建新的代理处理器
    pub fn new(config: Config) -> Self {
        let price_manager = PriceManager::from_config(&config);
        let tolerance = config.audit.tolerance;

        Self {
            config,
            price_manager,
            token_calculator: TokenCalculator::new().unwrap_or_default(),
            cost_calculator: CostCalculator::new(tolerance),
            terminal_display: TerminalDisplay::new(),
            onboarding: OnboardingDisplay::new(),
            request_cache: RequestCache::new(),
        }
    }

    /// 处理代理请求
    pub async fn handle_request(&self, req: Request<Body>) -> Result<Response<Body>> {
        // 检查是否为 CONNECT 方法（HTTPS 代理）
        if req.method() == Method::CONNECT {
            return self.handle_connect(req).await;
        }

        // 处理普通 HTTP 请求
        self.handle_http_request(req).await
    }

    /// 处理 CONNECT 请求（建立 HTTPS 隧道）
    async fn handle_connect(&self, req: Request<Body>) -> Result<Response<Body>> {
        let target = req.uri().to_string();
        info!("CONNECT 请求，目标: {}", target);

        // 解析目标地址
        let target_addr = if target.contains(':') {
            target.clone()
        } else {
            format!("{}:443", target)
        };

        info!("正在连接到: {}", target_addr);

        // 连接到目标服务器
        let server_stream = match tokio::net::TcpStream::connect(&target_addr).await {
            Ok(stream) => stream,
            Err(e) => {
                error!("无法连接到目标服务器 {}: {}", target_addr, e);
                return Ok(Response::builder()
                    .status(StatusCode::BAD_GATEWAY)
                    .body(Body::from(format!("无法连接到目标服务器: {}", e)))
                    .unwrap());
            }
        };

        info!("成功连接到: {}", target_addr);

        // 返回 200 表示隧道建立成功
        let response = Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap();

        // 升级 HTTP 连接并启动异步代理任务
        tokio::spawn(async move {
            match hyper::upgrade::on(req).await {
                Ok(mut upgraded) => {
                    // 使用 copy_bidirectional 进行双向代理
                    match tokio::io::copy_bidirectional(
                        &mut tokio::io::BufReader::new(server_stream),
                        &mut tokio::io::BufWriter::new(upgraded),
                    )
                    .await
                    {
                        Ok((client_to_server, server_to_client)) => {
                            info!(
                                "HTTPS 隧道关闭 - 客户端->服务器: {} 字节, 服务器->客户端: {} 字节",
                                client_to_server, server_to_client
                            );
                        }
                        Err(e) => {
                            error!("HTTPS 隧道传输错误: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("连接升级失败: {}", e);
                }
            }
        });

        Ok(response)
    }

    /// 处理普通 HTTP 请求
    async fn handle_http_request(&self, req: Request<Body>) -> Result<Response<Body>> {
        let start_time = Instant::now();

        // 提取请求信息
        let method = req.method().clone();
        let uri = req.uri().clone();
        let headers = req.headers().clone();

        info!("处理 HTTP 请求: {} {}", method, uri);

        // 收集请求体
        let body_bytes = hyper::body::to_bytes(req.into_body()).await?;

        // 转发请求到目标服务器
        let (status, response_headers, response_body) =
            forwarder::forward_request(&method, &uri, &headers, &body_bytes).await?;

        // 计算总耗时
        let total_ms = start_time.elapsed().as_millis() as u64;

        // 审计分析（仅对 API 响应）
        if uri.path().contains("/chat/completions") || uri.path().contains("/messages") {
            if let Err(e) = self
                .analyze_response(&uri, &response_body, total_ms, status)
                .await
            {
                warn!("审计分析失败: {}", e);
            }
        }

        // 构建响应
        let mut response_builder = Response::builder().status(status);

        // 复制响应头
        for (key, value) in response_headers.iter() {
            response_builder = response_builder.header(key, value);
        }

        let response = response_builder.body(Body::from(response_body)).unwrap();

        Ok(response)
    }

    /// 分析 API 响应
    async fn analyze_response(
        &self,
        uri: &hyper::Uri,
        response_body: &str,
        total_ms: u64,
        status_code: u16,
    ) -> Result<()> {
        // 尝试解析 JSON 响应
        let json: Value = match serde_json::from_str(response_body) {
            Ok(v) => v,
            Err(_) => return Ok(()), // 不是 JSON，跳过
        };

        // 提取模型名称
        let model = json["model"].as_str().unwrap_or("unknown").to_string();

        // 提取 Hub 地址
        let hub = uri.host().unwrap_or("unknown").to_string();

        // 提取 Token 使用量
        let usage = &json["usage"];
        let input_tokens = usage["prompt_tokens"].as_u64().unwrap_or(0) as usize;
        let output_tokens = usage["completion_tokens"].as_u64().unwrap_or(0) as usize;
        let total_tokens = usage["total_tokens"].as_u64().unwrap_or(0) as usize;

        // 如果 API 没有返回 usage，尝试本地计算
        let stats = if total_tokens > 0 {
            crate::audit::token::TokenStats {
                input_tokens,
                output_tokens,
                total_tokens,
            }
        } else {
            // TODO: 从请求体和响应体计算 Token
            crate::audit::token::TokenStats {
                input_tokens: 0,
                output_tokens: 0,
                total_tokens: 0,
            }
        };

        // 计算成本（如果启用了审计模式）
        let cost_usd = if let Some(price) = self.price_manager.get_price(&model) {
            let cost_result = self.cost_calculator.calculate_cost(&stats, price);
            cost_result.total_cost
        } else {
            0.0
        };

        // 估算 TTFT（简化：假设为首响应的 10%）
        let ttft_ms = total_ms / 10;

        // 计算吐字速率
        let tpot = if total_ms > 0 {
            (output_tokens as f64) / (total_ms as f64 / 1000.0)
        } else {
            0.0
        };

        // 显示结果
        self.onboarding.show_once();

        if self.price_manager.is_audit_enabled() && cost_usd > 0.0 {
            // 成本审计模式
            if let Some(price) = self.price_manager.get_price(&model) {
                let cost_result = self.cost_calculator.calculate_cost(&stats, price);
                self.terminal_display.display_audit(
                    &model,
                    &hub,
                    &stats,
                    &cost_result,
                    ttft_ms,
                    total_ms,
                    tpot,
                );

                // 检查是否超过告警阈值
                if cost_result.total_cost > self.config.display.alert_threshold {
                    self.terminal_display.display_cost_alert(
                        cost_result.total_cost,
                        self.config.display.alert_threshold,
                    );
                }
            }
        } else {
            // 实时监测模式
            self.terminal_display
                .display_monitor(&model, &hub, &stats, ttft_ms, total_ms, tpot);
        }

        // 缓存请求记录
        self.request_cache.add_record(RequestRecord {
            timestamp: std::time::SystemTime::now(),
            model,
            hub,
            stats,
            cost_usd,
            latency_ms: total_ms,
            ttft_ms,
            tpot,
            status_code,
        });

        Ok(())
    }
}
