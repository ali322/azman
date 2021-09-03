use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Access {
    pub perm_id: Vec<i32>,
    pub role_id: i32,
}
