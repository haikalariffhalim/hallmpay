use crate::client::PayNetClient;
use crate::error::PayNetError;
use crate::request::RetryStrategy;
use secrecy::SecretString;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Account {
    pub id: String,
}

#[async_trait::async_trait]
pub trait AccountsApi {
    async fn get_account(&self, secret_key: &SecretString) -> Result<Account, PayNetError>;
}

#[async_trait::async_trait]
impl AccountsApi for PayNetClient {
    async fn get_account(&self, secret_key: &SecretString) -> Result<Account, PayNetError> {
        self.get("/account", secret_key, RetryStrategy::default())
            .await
    }
}
