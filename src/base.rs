pub fn default_request_builder() -> http::request::Builder {
    http::Request::builder()
        .header("Accept", "application/json")
}

pub fn sign_request(
    builder: http::request::Builder,
    url: &url::Url,
    body_json: Option<&str>,
    auth: &crate::context::AuthContext
) -> http::request::Builder {
    let timestamp = chrono::Utc::now().timestamp();
    let message = format!("{}{}{}", url, timestamp, body_json.unwrap_or("{}"));
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
