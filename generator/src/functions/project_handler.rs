use crate::structs::project::Project;
use reqwest::Error;

pub fn get_project(ip: String, port: u16) -> Project {
    Project {
        name: "Eva & Leila".to_string(),
        ip,
        port,
    }
}

pub async fn register_project(url: &str, project: &Project) -> Result<(), Error> {
    let client = reqwest::Client::new();
    client.post(url).json(project).send().await?;
    Ok(())
}
