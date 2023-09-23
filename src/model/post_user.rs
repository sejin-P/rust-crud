use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PostUser {
    pub title: String,
    pub body: String,
    pub user_name: String,
    pub user_email: String,
}