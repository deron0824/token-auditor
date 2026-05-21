use anyhow::Result;
use hyper::{HeaderMap, Method, Uri};
use tracing::{error, info};

/// 转发请求到目标服务器
pub async fn forward_request(
    method: &Method,
    uri: &Uri,
    headers: &HeaderMap,
    body: &[u8],
) -> Result<(u16, HeaderMap, String)> {
    // 构建目标 URL
    let target_url = build_target_url(uri)?;

    info!("转发请求到: {}", target_url);

    // 创建 HTTP 客户端
    let client = reqwest::Client::new();

    // 构建请求
    let mut request_builder = client.request(method.clone(), &target_url);

    // 复制请求头（跳过代理相关的头）
    for (name, value) in headers.iter() {
        if name != "proxy-connection" && name != "host" {
            request_builder = request_builder.header(name.as_str(), value);
        }
    }

    // 添加请求体
    if !body.is_empty() {
        request_builder = request_builder.body(body.to_vec());
    }

    // 发送请求
    let response = match request_builder.send().await {
        Ok(resp) => resp,
        Err(e) => {
            error!("转发请求失败: {}", e);
            return Err(e.into());
        }
    };

    // 提取响应状态码
    let status = response.status().as_u16();

    // 提取响应头
    let response_headers = response.headers().clone();

    // 读取响应体
    let response_body = response.text().await?;

    Ok((status, response_headers, response_body))
}

/// 构建目标 URL
fn build_target_url(uri: &Uri) -> Result<String> {
    // 如果 URI 已经包含完整的 scheme 和 authority，直接使用
    if uri.scheme().is_some() && uri.authority().is_some() {
        return Ok(uri.to_string());
    }

    // 否则从 Host 头构建
    // 这里简化处理，假设是 HTTP
    Err(anyhow::anyhow!("无法构建目标 URL: {}", uri))
}
