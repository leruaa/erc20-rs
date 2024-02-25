use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use tower::Service;

use crate::{error::InternalError, stores::TokenStore, Error, Token, TokenClient, TokenId};

pub struct TokenService {
    chain_id: u8,
    client: Arc<TokenClient>,
    store: Box<Arc<dyn TokenStore + Send + Sync>>,
}

impl TokenService {
    pub fn new(
        chain_id: u8,
        client: Arc<TokenClient>,
        store: Arc<dyn TokenStore + Send + Sync>,
    ) -> Self {
        Self {
            chain_id,
            client,
            store: Box::new(store),
        }
    }
}

impl Service<TokenId> for TokenService {
    type Response = Arc<Token>;

    type Error = Error;

    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, id: TokenId) -> Self::Future {
        let chain_id = self.chain_id;
        let client = self.client.clone();
        let store = self.store.clone();

        let fut = async move {
            if let Some(token) = store.get(chain_id, id.clone()) {
                Ok(token)
            } else {
                match id {
                    TokenId::Symbol(_) => Err(Error::new(id, InternalError::NotInStore)),
                    TokenId::Address(address) => match client.retrieve_token(address).await {
                        Ok(token) => {
                            let token = Arc::new(token);
                            store.insert(chain_id, token.clone());
                            Ok(token)
                        }
                        Err(err) => Err(err),
                    },
                }
            }
        };

        Box::pin(fut)
    }
}