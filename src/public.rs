use super::base;
use super::extractor;

pub struct KunaPublicClient<TConnector> {
    client: std::sync::Arc<hyper::Client<TConnector>>,
    base_url: url::Url,
}

impl<TConnector> KunaPublicClient<TConnector> 
where 
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static
{
    pub fn new(
        client: std::sync::Arc<hyper::Client<TConnector>>,
        base_url: url::Url,
    ) -> KunaPublicClient<TConnector> {
        KunaPublicClient {
            client,
            base_url,
        }
    }

    pub async fn get_orderbook(
        &self,
        coins: crate::coin::Coins
    ) -> Result<crate::models::OrderBook, String> {
        let coins_string = coins.to_string();
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .expect("Invalid url")
            .push(base::VERSION)
            .push(base::BOOK)
            .push(&coins_string);
        let request = match base::default_request_builder(&url)
            .method(hyper::Method::GET)
            .body(hyper::Body::empty()) {
            Ok(request) => request,
            Err(error) => return Err(format!("Failed to create request: {:#?}", error)),
        };
        let (_header, body) = match self
            .client
            .request(request)
            .await {
            Ok(response) => response.into_parts(),
            Err(error) => return Err(format!("Failed to create response: {:#?}", error)),
        };
        use crate::models::OrderBookEntries;
        let order_book_entries = match extractor::read_body::<OrderBookEntries>(body)
            .await {
            Some(order_book) => order_book,
            None => return Err("Failed to read body: {:#?}".to_owned()),
        };
        Ok(crate::models::OrderBook::with(coins, order_book_entries))
    }

    pub async fn get_markets(&self) -> Result<crate::models::Markets, String> {
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .expect("Invalid url")
            .push(base::VERSION)
            .push(base::MARKETS);
        let request = match base::default_request_builder(&url)
            .method(hyper::Method::GET)
            .body(hyper::Body::empty()) {
            Ok(request) => request,
            Err(error) => return Err(format!("Failed to create request: {:#?}", error)),
        };
        let (_header, body) = match self.client
            .request(request)
            .await {
            Ok(response) => response.into_parts(),
            Err(error) => return Err(format!("Failed to create response: {:#?}", error)),
        };
        match extractor::read_body::<crate::models::Markets>(body).await {
            Some(result) => Ok(result),
            None => Err("Failed to deserailize the body".to_owned()),
        }
    }
}
