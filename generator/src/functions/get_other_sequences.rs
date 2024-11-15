use crate::errors::CustomError;
use crate::functions::http_handler::create_error_response;
use crate::structs::project::Project;
use crate::structs::sequence::{SequenceInfo, SequenceRequest};

use reqwest::{Client, StatusCode};
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
    let generators = fetch_registered_generators(register_url)
    .await
    .map_err(|e| {
        create_error_response(
            CustomError::UnknownError(format!("Failed to fetch generators: {}", e)),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;;
    let client = Client::new();
    let mut available_generators = HashMap::new();

    for generator in generators {
        let url = format!("http://{}:{}/sequence", generator.ip, generator.port);
        let response = client.get(&url).send().await;

        if let Ok(resp) = response {
            if resp.status().is_success() {
                let supported_sequences = resp.json::<Vec<SequenceInfo>>()
                    .await
                    .map_err(|e| {
                        create_error_response(
                            CustomError::InvalidInputFormat(format!("Invalid JSON from {}:{} - {}", generator.ip, generator.port, e)),
                            StatusCode::BAD_REQUEST,
                        )
                    })?;
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
            } else {
                return create_error_response(
                    CustomError::UnresponsiveSequence {
                        name: sequence_names.join(", "),
                        projects: vec![generator.name.clone()],
                    },
                    StatusCode::BAD_GATEWAY,
                );
            }
        } else {
            return create_error_response(
                CustomError::UnresponsiveSequence {
                    name: sequence_names.join(", "),
                    projects: vec![generator.name.clone()],
                },
                StatusCode::BAD_GATEWAY,
            );
        }
    }
    Ok(available_generators)
}

pub async fn pull_sequence(register_url: &str, name: &String, seq_request: SequenceRequest) -> Result<Vec<f64>, Box<dyn Error>> {
    let map = check_available_sequences(register_url, vec![name.clone()]).await?;
    let mut int_elems: Vec<f64> = Vec::new();

    if !map.is_empty() {
        let mut k = map.keys();
        let generator_opt = k.next();
        let client = Client::new();
        match generator_opt {
            Some(generator) => {
                let url = format!("http://{}:{}/sequence/{}", generator.ip, generator.port, name);
                let response = client.post(&url).json(&seq_request).send().await;

                if let Ok(resp) = response {
                    //let json: Value = response.json().await?;

                    // let elems: String = ""; // resp.range al neki
                    // let arr_elems: SplitWhitespace<'_> = elems.split_whitespace();
                    // for elem in arr_elems {
                    //     int_elems.push(elem.parse::<f64>().unwrap());
                    // }
                    
                    // ZA PRETVARJNANJE STRUCTA V JSON let seq_request_json = serde_json::to_string(&seq_request); 
                    int_elems = resp.json::<Vec<f64>>().await?;




                }
            }
            None => panic!()

                
        }


    }

    Ok(int_elems)



}


/*  if map.is_empty() {
        return create_error_response(
            CustomError::InvalidSequenceName(name.clone()),
            StatusCode::NOT_FOUND,
        );
    }
    let mut generators = map.keys();
    if let Some(generator) = generators.next() {
        let client = Client::new();
        let url = format!(
            "http://{}:{}/sequence/{}",
            generator.ip, generator.port, name
        );
        let response = client.post(&url).json(&seq_request).send().await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                int_elems = resp
                    .json::<Vec<f64>>()
                    .await
                    .map_err(|e| {
                        create_error_response(
                            CustomError::InvalidInputFormat(format!(
                                "Invalid JSON response from {}:{} - {}",
                                generator.ip, generator.port, e
                            )),
                            StatusCode::BAD_REQUEST,
                        )
                    })?;
            }
            Ok(resp) => {
                return create_error_response(
                    CustomError::UnresponsiveSequence {
                        name: name.clone(),
                        projects: vec![generator.name.clone()],
                    },
                    resp.status(),
                );
            }
            Err(_) => {
                return create_error_response(
                    CustomError::UnresponsiveSequence {
                        name: name.clone(),
                        projects: vec![generator.name.clone()],
                    },
                    StatusCode::BAD_GATEWAY,
                );
            }
        }
    } else {
        return create_error_response(
            CustomError::InvalidSequenceName(name.clone()),
            StatusCode::NOT_FOUND,
        );
    }

    Ok(int_elems)
}

*/