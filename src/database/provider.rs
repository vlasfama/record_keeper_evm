use crate::model::*;

pub trait DatabaseReader {
    fn get_wallet(&self, chain_id: &str) -> anyhow::Result<Option<()>>;
    fn get_address(&self, user_id: &str, chain_id: &str) -> anyhow::Result<Option<()>>;
    fn has_address(&self, address: &str, chain_id: &str) -> anyhow::Result<bool>;
}
pub trait DatabaseWriter {
    fn create_user(
        &self,
        user_id: &str,
        chain_id: &str,
        wallet_id: &str,
        wallet_address: &str,
    ) -> anyhow::Result<()>;

    fn remove_user(
        &self,
        user_id: &str,
        chain_id: &str,
        wallet_id: &str,
        wallet_address: &str,
    ) -> anyhow::Result<()>;

    fn update_user(
        &self,
        user_id: &str,
        chain_id: &str,
        wallet_id: &str,
        wallet_address: &str,
    ) -> anyhow::Result<()>;
}

pub trait DatabaseProvider: DatabaseReader + DatabaseWriter {}

impl<T: DatabaseReader + DatabaseWriter> DatabaseProvider for T {}
