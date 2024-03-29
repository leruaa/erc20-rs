use std::sync::Arc;

use crate::{util::StoreIter, Token, TokenId};

mod basic;

use alloy_primitives::Address;
pub use basic::BasicTokenStore;

/// A [`Token`] store
pub trait TokenStore: Sized {
    /// Inserts a token into the store.
    fn insert(&mut self, chain_id: u8, token: Arc<Token>);

    /// Returns `true` if the store contains a value for the specified `id`.
    fn contains(&self, chain_id: u8, id: TokenId) -> bool;

    /// Returns the value corresponding to the given id.
    fn get(&self, chain_id: u8, id: TokenId) -> Option<Arc<Token>>;
    fn symbols(&self, chain_id: Option<u8>) -> impl Iterator<Item = String>;
    fn addresses(&self, chain_id: Option<u8>) -> impl Iterator<Item = Address>;

    fn iter(&self, chain_id: u8) -> StoreIter<Self> {
        StoreIter::new(self, chain_id)
    }

    #[cfg(feature = "known-tokens")]
    fn insert_known_tokens(&mut self, chain_id: u8) {
        use crate::mainnet;

        if chain_id == 1 {
            self.insert(chain_id, Arc::new(mainnet::WETH.to_owned()));
            self.insert(chain_id, Arc::new(mainnet::WBTC.to_owned()));
            self.insert(chain_id, Arc::new(mainnet::USDC.to_owned()));
            self.insert(chain_id, Arc::new(mainnet::USDT.to_owned()));
            self.insert(chain_id, Arc::new(mainnet::DAI.to_owned()));
        }
    }
}
