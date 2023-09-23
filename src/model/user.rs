use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct User {
    pub name: String,
    #[serde(default)]
    pub email: String,
    pub age: u8,
}
