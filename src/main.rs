use std::io::prelude::*;
use std::io::Result;
use std::net::{TcpListener, TcpStream};

fn main() -> Result<()> {
	let server = TcpListener::bind("0.0.0.0:7070")?;

	for stream in server.incoming() {
		let stream = stream?;
		handle_connection(stream)?;
	}

	Ok(())
}

use std::path::PathBuf;

fn handle_connection(mut stream: TcpStream) -> Result<()> {
	let mut buffer = [0; 512];
	stream.read(&mut buffer)?;

	let request = String::from_utf8_lossy(&buffer);
	println!("{} is requesting resource", stream.peer_addr()?);
	let line = request.lines().next().unwrap();

	let response = match handle_request(&line) {
		Ok(response) => response,
		Err(e) => panic!("{}", e),
	};

	println!("Sending response to {}", stream.peer_addr()?);
	stream.write(response.as_bytes())?;
	stream.flush()?;

	Ok(())
}

use std::fs;

fn handle_request(request: &str) -> Result<String> {
	let mut parts = request.split_whitespace();
	let _method = parts.next().unwrap_or("Method not specified");
	let uri = parts.next().unwrap_or("URI not specified");
	let _http_version = parts.next().unwrap_or("HTTP Version not specified");

	let uri = PathBuf::from(uri);
	// let request = Request::new(method, uri, http_version);

	let root: PathBuf = PathBuf::from("assets/");
	let resource = root.join(&uri);

	let mut response = String::new();
	if uri == PathBuf::from("/") {
		let contents = fs::read_to_string("assets/index.html")?;
		response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);
	}
	else if resource.exists() {
		let contents = fs::read_to_string(&resource)?;
		response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);
	}
	else {
		println!("Cannot locate requested resource");
	}

	Ok(response)
}

#[derive(Debug)]
struct Request {
	method: String,
	uri: PathBuf,
	http_version: String,
}

impl Request {
	fn new(method: impl Into<String>, uri: PathBuf, http_version: impl Into<String>) -> Request {
		let method = method.into();
		let uri = uri;
		let http_version = http_version.into();
		Request {
			method,
			uri,
			http_version,
		}
	}
}