use crate::model::User;
use anyhow::Result;

pub trait DatabaseReader {
    fn get_info(&self, user: &User)->Result<()>;
}
pub trait DatabaseWriter {
    fn create_user(&self, user: User)->Result<()>;

    fn update_user(&self, user: User)->Result<()>;
}

pub trait DatabaseProvider: DatabaseReader + DatabaseWriter {}

impl<T: DatabaseReader + DatabaseWriter> DatabaseProvider for T {}

