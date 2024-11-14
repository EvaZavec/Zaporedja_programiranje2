use crate::structs::project::Project;
use crate::structs::sequence::SequenceInfo;
use reqwest::Client;
use std::collections::HashMap;
use std::error::Error;

pub async fn fetch_registered_generators(register_url: &str) -> Result<Vec<Project>, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(register_url).send().await?;
    let generators = response.json::<Vec<Project>>().await?;
    Ok(generators)
}

pub async fn check_available_sequences(
    register_url: &str,
    sequence_names: Vec<String>,
) -> Result<HashMap<Project, Vec<String>>, Box<dyn Error>> {
    let generators = fetch_registered_generators(register_url).await?;
    let client = Client::new();
    let mut available_generators = HashMap::new();

    for generator in generators {
        let url = format!("http://{}:{}/sequence", generator.ip, generator.port);
        let response = client.get(&url).send().await;

        if let Ok(resp) = response {
            if resp.status().is_success() {
                let supported_sequences = resp.json::<Vec<SequenceInfo>>().await?;
                let matching_sequences: Vec<String> = supported_sequences
                    .iter()
                    .filter_map(|seq| {
                        if sequence_names.contains(&seq.name) {
                            Some(seq.name.clone())
                        } else {
                            None
                        }
                    })
                    .collect();

                if !matching_sequences.is_empty() {
                    available_generators.insert(generator, matching_sequences);
                }
            }
        }
    }

    Ok(available_generators)
}
