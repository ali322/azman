use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Access {
    pub perm_id: Vec<String>,
    pub role_id: String,
}
