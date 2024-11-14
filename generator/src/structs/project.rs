use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Project {
    pub name: String,
    pub ip: String,
    pub port: u16,
}