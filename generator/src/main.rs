use std::env;
use std::net::SocketAddr;

use functions::handle_request;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

const DEFAULT_IP: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 9000;

use crate::errors::CustomError;
use crate::functions::send_get_post::{send_get, send_post};
use crate::functions::project_handler::get_project;

pub mod errors;
pub mod functions;
pub mod info;
pub mod sequence;
pub mod structs;


#[tokio::main]
async fn main() -> Result<(), CustomError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(CustomError::InvalidInputFormat(
            "Write: cargo run -- IP_REGISTRA [IP_GENERATORJA PORT]".to_string(),
        ));
    }
    
    let register_url = args[1].clone();
    let generator_ip = args.get(2).unwrap_or(&DEFAULT_IP.to_string()).clone();
    let generator_port = args
        .get(3)
        .and_then(|p| p.parse().ok())
        .unwrap_or(DEFAULT_PORT);

    let addr: SocketAddr = (generator_ip.parse::<std::net::IpAddr>().map_err(|_| {
        CustomError::InvalidInputFormat(format!("Invalid IP address: {}", generator_ip))
    })?, generator_port).into();

    let project = get_project(generator_ip.clone(), generator_port);

    println!("Checking register status at {}", register_url);
    if let Err(err) = send_get(&register_url).await {
        return Err(CustomError::UnknownError("Register not available".to_string()));
    }

    println!("Registering generator: {:?}", project);
    send_post(&register_url, serde_json::to_string(&project).map_err(|_| {
        CustomError::InvalidInputFormat("Failed to serialize project".to_string())
    })?).await?;
    

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
