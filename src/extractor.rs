pub async fn read_body<TResult>(body: hyper::Body) -> Option<TResult>
where
    TResult: serde::de::DeserializeOwned,
{
    let bytes = match hyper::body::to_bytes(body)
        .await {
        Ok(bytes) => bytes,
        Err(error) => {
            log::error!("Failed to get bytes from body: {:#?}", error);
            return None;
        }
    };
    match serde_json::from_slice(&bytes) {
        Ok(result) => Some(result),
        Err(error) => {
            log::error!("Error on reading the body: {:#?}", error);
            log::error!("Json: {:#?}", String::from_utf8(bytes.to_vec()));
            None
        }
    }
}
