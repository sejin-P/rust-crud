use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Post {
    pub title: String,
    pub body: String,
    pub user_id: u64,
}