use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub access_token: String,
}
