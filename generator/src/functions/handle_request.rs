use crate::info::sequences;
use crate::structs::sequence::{SequenceRequest, SequenceSyntax};
use crate::functions::project_handler::get_project;
use crate::functions::http_handler::{full, collect_body, create_404};
use crate::functions::seq_from_syntax::seq_from_syntax;
use crate::errors::CustomError;

use bytes::Bytes;
use hyper::{body::Incoming, Method, Request, Response};
use http_body_util::combinators::BoxBody;

fn to_syntax(name: &String, parameters: Vec<f64>, sequences: Vec<Box<SequenceSyntax>>) -> SequenceSyntax {
    SequenceSyntax {
        name : name.clone(),
        parameters,
        sequences,
    }
}

pub async fn handle_request(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
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
            let request: SequenceRequest = serde_json::from_str(&body).unwrap();
            let range = request.range;
            let wanted_sequences = request.sequences;
            let wanted_parameters = request.parameters;
            match sequence {
                Some(sequence_syntax) => {
                    let syntax = to_syntax(&sequence_syntax.name, wanted_parameters, wanted_sequences);
                    let seq = seq_from_syntax(&syntax);
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }
                None => create_404(),
            }
        }
        _ => create_404(),
    }
}
