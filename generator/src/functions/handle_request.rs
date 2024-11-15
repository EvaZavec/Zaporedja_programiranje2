use crate::info::sequences;
use crate::errors::CustomError;
use crate::functions::http_handler::{collect_body, create_404, create_200, create_error_response, full};
use crate::functions::project_handler::get_project;
use crate::functions::seq_from_syntax::seq_from_syntax;
use crate::structs::sequence::{SequenceRequest, SequenceSyntax};

use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{body::Incoming, Method, Request, Response};
use reqwest::StatusCode;

fn to_syntax(name: &String, parameters: Vec<f64>, sequences: Vec<Box<SequenceSyntax>>) -> SequenceSyntax {
    SequenceSyntax {
        name : name.clone(),
        parameters,
        sequences,
    }
}

pub async fn handle_request(
    req: Request<Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/ping") => Ok(Response::new(full(
            serde_json::to_string(&get_project()).unwrap(),
        ))),
        (&Method::GET, "/sequence") => {
            let sequences = sequences();
            Ok(Response::new(full(
                serde_json::to_string(&sequences).unwrap(),
            )))
        }
        (&Method::POST, r) => {
            let our_sequences = sequences();
            let sequence = our_sequences
                .iter()
                .find(|&x| ("/sequence/".to_string() + &x.name) == r);

            let body = collect_body(req).await?;
            let request: Result<SequenceRequest, _> = serde_json::from_str(&body);

            if let Err(_) = request {
                let error = CustomError::InvalidInputFormat("Invalid request format".to_string());
                return create_error_response(StatusCode::BAD_REQUEST, error.to_json());
            }

            let request = request.unwrap();
            let range = request.range;
            let wanted_sequences = request.sequences;
            let wanted_parameters = request.parameters;

            match sequence {
                Some(sequence_syntax) => {
                    let syntax = to_syntax(&sequence_syntax.name, wanted_parameters, wanted_sequences);
                    let seq = seq_from_syntax(&syntax, &range).await;

                    if seq.is_err() {
                        let error = CustomError::InvalidOperation("Failed to generate sequence".to_string());
                        return create_error_response(StatusCode::BAD_REQUEST, error.to_json());
                    }

                    let seq = seq.unwrap();
                    let sequence_values = seq.range(range); 
                    create_200(serde_json::to_string(&sequence_values).unwrap())
                }
                None => {
                    let error = CustomError::InvalidSequenceName(r.to_string());
                    create_error_response(StatusCode::BAD_REQUEST, error.to_json())
                }
            }
        }
        _ => create_404(),
    }
}
