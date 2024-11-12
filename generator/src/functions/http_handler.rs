use bytes::Bytes;
use hyper::{body::Incoming, body::Body, Method, Request, Response, StatusCode};
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use crate::info::sequences;
use crate::structs::sequence::SequenceRequest;
use crate::functions::project_handler::get_project;


pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

pub async fn collect_body(req: Request<Incoming>) -> Result<String, hyper::Error> {
    let max = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if max > 1024 * 64 {
        panic!("Body too big");
    }

    let whole_body = req.collect().await?.to_bytes();
    let whole_body = std::str::from_utf8(&whole_body).unwrap().to_string();
    Ok(whole_body)
}

fn create_404() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let mut not_found = Response::new(empty());
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}

fn create_200<T: Into<Bytes>>(body: T) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let mut ok = Response::new(full(body));
    *ok.status_mut() = StatusCode::OK;
    Ok(ok)
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
