use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupResponse {
    pub user_id: String,
    pub email_duplicate: bool,
}
