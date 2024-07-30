use crate::model::UserInfo;
use anyhow::Result;
pub trait DatabaseReader {
    fn get_info(&self, user: &str) -> Result<(UserInfo)>;
}
pub trait DatabaseWriter {
    fn create_user(&self, user: UserInfo) -> Result<()>;

    fn update_user(&self, user: UserInfo) -> Result<()>;
}

pub trait DatabaseProvider: DatabaseReader + DatabaseWriter {}

impl<T: DatabaseReader + DatabaseWriter> DatabaseProvider for T {}
