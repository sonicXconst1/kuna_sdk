pub struct KunaClient<TConnector> {
    client: std::sync::Arc<hyper::Client<TConnector>>,
    auth_context: std::sync::Arc<crate::context::AuthContext>,
}

impl<TConnector> KunaClient<TConnector> 
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static
{
    const VERSION: &'static str = "v3";
    const AUTH: &'static str = "auth";
    const REQUEST: &'static str = "r";
    const WALLETS: &'static str = "wallets";

    pub fn new(
        client: std::sync::Arc<hyper::Client<TConnector>>,
        auth_context: std::sync::Arc<crate::context::AuthContext>,
    ) -> KunaClient<TConnector> {
        KunaClient {
            client,
            auth_context,
        }
    }

    pub async fn get_balance(&self) -> Result<Vec<agnostic::currency::Currency>, String> {
        unimplemented!()
    }
}
