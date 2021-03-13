use super::base;
use super::extractor;
use crate::models;

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
        let (_header, body) = self
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

    pub async fn create_order(
        &self,
        order: crate::models::CreateOrder
    ) -> Result<models::CreateOrderResponse, String> {
        let mut url = self.auth_context.base_url.clone();
        url.path_segments_mut()
            .expect("Invalid url")
            .push(base::VERSION)
            .push(base::AUTH)
            .push(base::W)
            .push(base::ORDER)
            .push(base::SUBMIT);
        let body = serde_json::to_string(&order).expect("Serialization error");
        let request = base::sign_request(
            base::default_request_builder(&url),
            &url,
            Some(&body),
            &self.auth_context,
        )
        .header("Content-Type", "application/json")
        .method(hyper::Method::POST)
        .body(hyper::Body::from(body))
        .expect("Failed to create request");
        let (_header, body) = self
            .client
            .request(request)
            .await
            .expect("Failed to send request")
            .into_parts();
        let body = match extractor::read_body::<models::CreateOrderResponseRaw>(body)
            .await {
            Some(body) => body,
            None => return Err("Failed to read body".to_owned()),
        };
        use std::convert::TryFrom;
        match models::CreateOrderResponse::try_from(body) {
            Ok(response) => Ok(response),
            Err(error) => Err(format!("{:#?}", error)),
        }
    }

    pub async fn delete_order(
        &self,
        cancel_order: crate::models::CancelOrderRequest,
    ) -> Result<crate::order::CanceledOrder, String> {
        let mut url = self.auth_context.base_url.clone();
        url.path_segments_mut()
            .expect("Invalid url")
            .push(base::VERSION)
            .push(base::ORDER)
            .push(base::CANCEL);
        let body = serde_json::to_string(&cancel_order).expect("Serialization error");
        let request = base::sign_request(
            base::default_request_builder(&url),
            &url,
            Some(&body),
            &self.auth_context,
        )
        .method(hyper::Method::POST)
        .body(hyper::Body::from(body))
        .expect("Failed to create request");
        let (header, body) = self
            .client
            .request(request)
            .await
            .expect("Failed to send request")
            .into_parts();
        let body_result = extractor::read_body::<crate::models::CanceledOrderResponse>(body)
            .await
            .expect("Failed to read the body");
        use std::convert::TryFrom;
        match crate::order::CanceledOrder::try_from(body_result) {
            Ok(cancel_order) => Ok(cancel_order),
            Err(error) => Err(format!("Header: {:#?}\nError: {}", header, error)),
        }
    }
}
