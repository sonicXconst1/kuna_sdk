use hmac::NewMac;
use hmac::Mac;

pub struct AuthContext {
    private_key: String,
    pub public_key: String,
    pub base_url: url::Url,
}

impl AuthContext {
    pub fn new(
        private_key: String,
        public_key: String,
        base_url: url::Url
    ) -> AuthContext {
        AuthContext {
            private_key,
            public_key,
            base_url,
        }
    }

    pub fn sign(&self, message: &str) -> String {
        let mut signature = hmac::Hmac::<sha2::Sha384>::new_from_slice(
            self.private_key.as_bytes())
            .expect("HMAC can take key of any size");
        signature.update(message.as_bytes());
        let signature = signature.finalize();
        hex::encode(signature.into_bytes())
    }
}
