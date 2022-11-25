use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReponse {
    pub server_ok: bool,
    pub database_ok: bool,
}
