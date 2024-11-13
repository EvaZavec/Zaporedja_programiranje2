use std::net::SocketAddr;

use functions::handle_request;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

const PORT: u16 = 12345;
//spremeni v prebrano iz command lina, podobno za ip

use crate::functions::send_get_post::{send_get, send_post};
use crate::functions::project_handler::get_project;

pub mod errors;
pub mod functions;
pub mod info;
pub mod sequence;
pub mod structs;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([0, 0, 0, 0], PORT).into();

    let b = send_get("http://0.0.0.0:7878/project".to_string()).await?;
    println!("HERE {}", b);

    let b = send_post(
        "http://0.0.0.0:7878/project".to_string(),
        serde_json::to_string(&get_project()).unwrap(),
    )
    .await?;
    println!("HERE {}", b);

    let b = send_get("http://0.0.0.0:7878".to_string()).await?;
    println!("HERE {}", b);

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let service = service_fn(move |req| {
            handle_request::handle_request(req)
        });

        if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
            println!("Error serving connection: {:?}", err);
        }
    }
}