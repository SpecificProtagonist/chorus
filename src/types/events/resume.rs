use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GatewayResume {
    pub token: String,
    pub session_id: String,
    pub seq: String,
}
