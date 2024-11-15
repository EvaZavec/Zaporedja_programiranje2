use bytes::Bytes;
use hyper::{body::Body, body::Incoming, Request, Response, StatusCode};
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};

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

pub fn create_404() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let mut not_found = Response::new(empty());
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}

pub fn create_200<T: Into<Bytes>>(body: T) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let mut ok = Response::new(full(body));
    *ok.status_mut() = StatusCode::OK;
    Ok(ok)
}

pub fn create_error_response<T: Into<Bytes>>(
    status: StatusCode,
    body: T,
) -> Result<Response<dyn Body>, hyper::Error> {
    let mut response = Response::new(Body::from(body.into()));
    *response.status_mut() = status;
    Ok(response)
}