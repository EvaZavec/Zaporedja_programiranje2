use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum CustomError {
    #[serde(rename = "InvalidInputFormat")]
    InvalidInputFormat(String),
    
    #[serde(rename = "InvalidSequenceName")]
    InvalidSequenceName(String),
    
    #[serde(rename = "UnresponsiveSequence")]
    UnresponsiveSequence {
        name: String,
        projects: Vec<String>,
    },
    
    #[serde(rename = "InvalidOperation")]
    InvalidOperation(String),
    
    #[serde(rename = "UnknownError")]
    UnknownError(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::InvalidInputFormat(msg) => write!(f, "Invalid input format: {}", msg),
            CustomError::InvalidSequenceName(msg) => write!(f, "Invalid sequence name: {}", msg),
            CustomError::UnresponsiveSequence { name, projects } => {
                write!(f, "Unresponsive sequence '{}', projects: {:?}", name, projects)
            }
            CustomError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            CustomError::UnknownError(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for CustomError {}

impl CustomError {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|_| "{\"error\": \"Unknown error\"}".to_string())
    }
}