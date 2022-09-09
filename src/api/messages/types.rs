use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Message {
    pub metadata: Metadata,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Metadata {
    pub api: String,
    pub branch: String,
}
