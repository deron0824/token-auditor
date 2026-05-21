use anyhow::Result;
use clap::Parser;
use token_auditor::cli::{Cli, Commands};
use token_auditor::config::Config;
use token_auditor::proxy;
use token_auditor::storage;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("token_auditor=info".parse()?),
        )
        .init();

    // 解析 CLI 参数
    let cli = Cli::parse();

    // 加载配置
    let config = Config::load();

    match cli.command {
        Commands::Start { listen } => {
            info!("启动 TokenAuditor 代理");
            info!("监听地址: {}", listen);

            // 创建代理处理器
            let handler = proxy::handler::ProxyHandler::new(config.clone());
            let handler_arc = std::sync::Arc::new(handler);

            // 提前克隆需要的值，避免被移动
            let snapshot_path = config.storage.snapshot_path.clone();
            let snapshot_interval = config.storage.snapshot_interval;

            println!("✓ TokenAuditor 已启动");
            println!("📡 监听地址: {}", listen);
            println!("📝 按 Ctrl+C 停止");

            // 启动代理服务器和快照定时任务
            let snapshot_path_clone = snapshot_path.clone();
            let snapshot_handler = tokio::spawn({
                let handler = handler_arc.clone();
                async move {
                    use std::time::Duration;
                    let mut interval_timer =
                        tokio::time::interval(Duration::from_secs(snapshot_interval));
                    loop {
                        interval_timer.tick().await;
                        let summary = handler.request_cache.calculate_summary();
                        if summary.total_requests > 0 {
                            if let Ok(snapshot_mgr) = storage::snapshot::SnapshotManager::new(
                                &snapshot_path_clone,
                                snapshot_interval,
                            ) {
                                if let Err(e) = snapshot_mgr.generate_snapshot(&summary) {
                                    tracing::warn!("生成快照失败: {}", e);
                                }
                            }
                        }
                    }
                }
            });

            // 启动代理
            let proxy_future = proxy::server::start_proxy(&listen, handler_arc.clone());

            // 等待 Ctrl+C
            tokio::select! {
                result = proxy_future => {
                    if let Err(e) = result {
                        eprintln!("❌ 代理启动失败: {}", e);
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    println!("\n👋 正在停止 TokenAuditor...");

                    // 生成最终快照
                    let handler = &*handler_arc;
                    let summary = handler.request_cache.calculate_summary();
                    if summary.total_requests > 0 {
                        if let Ok(snapshot_mgr) = storage::snapshot::SnapshotManager::new(
                            &snapshot_path,
                            snapshot_interval,
                        ) {
                            if let Err(e) = snapshot_mgr.generate_snapshot(&summary) {
                                eprintln!("⚠️  保存快照失败: {}", e);
                            } else {
                                println!("✓ 快照已保存");
                            }
                        }
                    }

                    println!("👋 TokenAuditor 已停止");
                }
            }
        }

        Commands::Benchmark {
            model,
            hubs,
            iterations,
        } => {
            info!("运行基准测试");
            info!("模型: {}", model);
            info!("Hubs: {}", hubs);
            info!("测试次数: {}", iterations);

            // TODO: 实现基准测试逻辑
            println!("🔧 基准测试功能开发中...");
        }

        Commands::Stats { snapshot } => {
            info!("查看历史统计");

            // TODO: 实现统计查看逻辑
            println!("📊 历史统计功能开发中...");
        }

        Commands::Export { output } => {
            info!("导出 JSON 快照");
            info!("输出文件: {}", output);

            // TODO: 实现导出逻辑
            println!("📤 导出功能开发中...");
        }
    }

    Ok(())
}
