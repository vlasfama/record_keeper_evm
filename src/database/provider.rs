use crate::model::User;

pub trait DatabaseReader {
    fn get_info(&self, user: &User);
}
pub trait DatabaseWriter {
    fn create_user(&self, user: User);

    fn update_user(&self, user: User);
}

pub trait DatabaseProvider: DatabaseReader + DatabaseWriter {}

impl<T: DatabaseReader + DatabaseWriter> DatabaseProvider for T {}
