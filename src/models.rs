use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}