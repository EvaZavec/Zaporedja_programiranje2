use crate::structs::project::Project;
use reqwest::Error;

pub fn get_project() -> Project {
    Project {
        name: "Eva & Leila".to_string(),
        ip: "0.0.0.0".to_string(),
        port: 12345,
    }
}

pub async fn register_project(url: &str, project: &Project) -> Result<(), Error> {
    let client = reqwest::Client::new();
    client.post(url).json(project).send().await?;
    Ok(())
}
