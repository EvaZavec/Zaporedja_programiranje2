use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Project {
    pub name: String,
    pub ip: String,
    pub port: u16,
}