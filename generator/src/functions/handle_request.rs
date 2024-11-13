use crate::sequence::arithmetic::Arithmetic;
use bytes::Bytes;
use hyper::{body::Incoming, Method, Request, Response};
use http_body_util::combinators::BoxBody;
use crate::info::sequences;
use crate::structs::sequence::SequenceRequest;
use crate::functions::project_handler::get_project;
use crate::functions::http_handler::{full,  collect_body, create_404, create_200};

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
            let seqs = sequences();
            let sequence = seqs
                .iter()
                .find(|&x| ("/sequence/".to_string() + &x.name) == r);
            match sequence {
                None => create_404(),
                Some(s) if *s.name == "Arithmetic" => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = Arithmetic::new(request.parameters[0], request.parameters[1]);
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }
                _ => panic!("Not implemented"),
            }
        }
        _ => create_404(),
    }
}
