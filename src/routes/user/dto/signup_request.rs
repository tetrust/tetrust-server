use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupRequest {
    pub nickname: String,
    pub email: String,
    pub password: String,
}
