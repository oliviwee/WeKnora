use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process;

use weknora_server::config::Config;
use weknora_server::http::HttpRequest;
use weknora_server::state::AppState;
use weknora_server::{handle_request, http::HttpResponse};

fn main() {
    if let Err(err) = run() {
        eprintln!("failed to start WeKnora Rust server: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    let addr = config.server_addr();
    let listener = TcpListener::bind(addr)?;
    let state = AppState::new(config);

    println!("starting WeKnora Rust server at http://{addr}");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream, &state),
            Err(err) => eprintln!("failed to accept connection: {err}"),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, state: &AppState) {
    let mut buffer = [0_u8; 8192];
    let response = match stream.read(&mut buffer) {
        Ok(0) => HttpResponse::bad_request("empty request"),
        Ok(size) => match HttpRequest::parse(&buffer[..size]) {
            Ok(request) => handle_request(&request, state),
            Err(err) => HttpResponse::bad_request(&err.to_string()),
        },
        Err(err) => HttpResponse::bad_request(&err.to_string()),
    };

    if let Err(err) = stream.write_all(&response.to_bytes()) {
        eprintln!("failed to write response: {err}");
    }
}
