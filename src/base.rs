pub const VERSION: &'static str = "v3";
pub const AUTH: &'static str = "auth";
pub const REQUEST: &'static str = "r";
pub const WALLETS: &'static str = "wallets";
pub const BOOK: &'static str = "book";

pub fn default_request_builder(url: &url::Url) -> http::request::Builder {
    http::Request::builder()
        .header("Accept", "application/json")
        .uri(url.to_string())
}

pub fn sign_request(
    builder: http::request::Builder,
    url: &url::Url,
    body_json: Option<&str>,
    auth: &crate::context::AuthContext
) -> http::request::Builder {
    let timestamp = chrono::Utc::now().timestamp_millis();
    log::info!("URL: {}", url.path());
    let body = body_json.unwrap_or("{}");
    log::info!("BODY: {}", body);
    let message = format!("{}{}{}", url.path(), timestamp, body_json.unwrap_or("{}"));
    builder
        .header("kun-nonce", timestamp)
        .header("kun-apikey", &auth.public_key)
        .header("kun-signature", auth.sign(&message))
}

pub fn create_request_with_body(
    builder: http::request::Builder,
    body: hyper::Body
) -> Result<http::Request<hyper::Body>, http::Error> {
    builder
        .header("Content-Type", "application/json")
        .body(body)
}
