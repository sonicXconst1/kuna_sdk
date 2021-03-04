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
        let request = base::default_request_builder(&url)
            .method(hyper::Method::GET)
            .body(hyper::Body::empty())
            .expect("Failed to create request");
        let (_header, body) = self
            .client
            .request(request)
            .await
            .expect("Failed to send request")
            .into_parts();
        use crate::models::OrderBookEntries;
        let order_book_entries = extractor::read_body::<OrderBookEntries>(body)
            .await
            .expect("Failed to read body");
        Ok(crate::models::OrderBook::with(
            coins,
            order_book_entries))
    }
}
