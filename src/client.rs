use super::base;
use super::extractor;

pub struct KunaClient<TConnector> {
    client: std::sync::Arc<hyper::Client<TConnector>>,
    auth_context: std::sync::Arc<crate::context::AuthContext>,
}

impl<TConnector> KunaClient<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{

    pub fn new(
        client: std::sync::Arc<hyper::Client<TConnector>>,
        auth_context: std::sync::Arc<crate::context::AuthContext>,
    ) -> KunaClient<TConnector> {
        KunaClient {
            client,
            auth_context,
        }
    }

    pub async fn get_balance(&self) -> Result<Vec<crate::models::Currency>, String> {
        let mut url = self.auth_context.base_url.clone();
        url.path_segments_mut()
            .expect("Invalid url")
            .push(base::VERSION)
            .push(base::AUTH)
            .push(base::REQUEST)
            .push(base::WALLETS);
        let request = base::sign_request(
            base::default_request_builder(&url),
            &url,
            None,
            &self.auth_context,
        )
        .method(hyper::Method::POST)
        .header("Content-Type", "application/json")
        .body(hyper::Body::from("{}"))
        .expect("Failed to create request");
        let (header, body) = self
            .client
            .request(request)
            .await
            .expect("Failed to send request")
            .into_parts();
        let currency = extractor::read_body::<crate::models::Currencies>(body)
            .await
            .expect("Failed to read body");
        let result: Vec<_> = currency
            .iter()
            .map(|currency_enties| {
                crate::models::Currency::from(currency_enties).expect("Failed to create currency")
            })
            .collect();
        Ok(result)
    }
}
