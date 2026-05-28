use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::process;
use std::time::Duration;

use weknora_rust::config::Config;
use weknora_rust::http::HttpRequest;
use weknora_rust::state::AppState;
use weknora_rust::{handle_request, http::HttpResponse};

fn main() {
    if env::args().any(|arg| arg == "--healthcheck") {
        if let Err(err) = run_healthcheck() {
            eprintln!("WeKnora-Rust healthcheck failed: {err}");
            process::exit(1);
        }
        return;
    }

    if let Err(err) = run() {
        eprintln!("failed to start WeKnora-Rust server: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    let addr = config.server_addr();
    let listener = TcpListener::bind(addr)?;
    let state = AppState::new(config);

    println!("starting WeKnora-Rust server at http://{addr}");
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

fn run_healthcheck() -> Result<(), Box<dyn std::error::Error>> {
    let host = env::var("WEKNORA_HEALTHCHECK_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("WEKNORA_SERVER_PORT")
        .unwrap_or_else(|_| "8080".into())
        .parse::<u16>()?;
    let timeout = Duration::from_secs(2);
    let addr = format!("{host}:{port}")
        .to_socket_addrs()?
        .next()
        .ok_or("healthcheck host did not resolve")?;

    let mut stream = TcpStream::connect_timeout(&addr, timeout)?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    stream.write_all(b"GET /healthz HTTP/1.1\r\nhost: localhost\r\nconnection: close\r\n\r\n")?;

    let mut response = [0_u8; 128];
    let size = stream.read(&mut response)?;
    let status_line = std::str::from_utf8(&response[..size])?
        .lines()
        .next()
        .unwrap_or_default();

    if status_line.contains(" 200 ") {
        Ok(())
    } else {
        Err(format!("unexpected healthcheck response: {status_line}").into())
    }
}
