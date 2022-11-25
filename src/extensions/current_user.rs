use crate::models::User;

#[derive(Debug, Clone, Default)]
pub struct CurrentUser {
    pub authorized: bool,
    pub user: Option<User>,
}
