//! reqwest 工具
//!
//! #Error
//! 可能的错误来源: 网络超时、DNS 解析失败、HTTP 请求失败、响应读取错误等.

use anyhow::Result;
use bytes::Bytes;
use reqwest::{
    Client, RequestBuilder,
    header::{ACCEPT, CONNECTION},
};
use std::sync::LazyLock;
use std::time::Duration;

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(60 * 30))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
        .build()
        .unwrap()
});

/// 配置 Request
///
/// # Errors
/// 此函数本身不会产生错误，但传入的 `RequestBuilder` 可能在后续发送请求时失败.
fn config_request(request: RequestBuilder) -> RequestBuilder {
    request
        .header(ACCEPT, "*/*")
        .header(CONNECTION, "Keep-Alive")
}

/// 发送 GET 请求
///
/// # Errors
/// - 网络超时或连接失败
/// - HTTP 状态码非 2xx 时 `reqwest` 可能返回错误
/// - 读取响应体失败
#[allow(dead_code)]
pub async fn get(url: &str) -> Result<Bytes> {
    let request = config_request(CLIENT.get(url));
    Ok(request.send().await?.bytes().await?)
}

/// 发送 POST 请求，消息体为 JSON
///
/// # Errors
/// - JSON 序列化失败
/// - 网络超时或连接失败
/// - HTTP 状态码非 2xx 时 `reqwest` 可能返回错误
/// - 读取响应体失败
#[allow(dead_code)]
pub async fn post_json<T: serde::Serialize + Sync + ?Sized>(
    url: &str,
    json_data: &T,
) -> Result<Bytes> {
    let mut request = config_request(CLIENT.post(url));
    request = request.json(json_data);
    Ok(request.send().await?.bytes().await?)
}
