use super::request::HttpRequest;
use super::response::HttpResponse;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use colored::*;

pub struct HttpServer<'a> {
    host: &'a str,
    port: u16
}

impl<'a> HttpServer<'a> {
    pub fn new(host: &'a str, port: u16) -> Self {
        Self {host, port}
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {
                let request = String::from_utf8_lossy(&buffer[..]);
                let http_request = HttpRequest::new(&request);
                println!("Method: {}", http_request.method);
                println!("Path: {}", http_request.path);
                for (key, value) in http_request.headers.iter() {
                    println!("Header: {}: {}", key, value);
                }
                println!("Body: {}", http_request.body);

                // Create a simple response
                let response = HttpResponse::new(200)
                    .add_header("Content-Type", "text/html")
                    .set_body("<html><body><h1>foobar</h1></body></html>".to_string())
                    .to_string();

                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            },
            Err(e) => {
                println!("{} {}", "Failed to read from connection:".red(), e.to_string().red())
            }
        }
    }

    pub fn listen(&self) {
        match TcpListener::bind((self.host, self.port)) {
            Ok(listener) => {
                println!(
                    "{} {}{}{}", // e.g. Listening on 127.0.0.1:8080
                    "Listening on".green(),
                    self.host.green(),
                    ":".green(),
                    self.port.to_string().green()
                );
                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            let _ = &self.handle_client(stream);
                        },
                        Err(error) => {
                            println!("{} {}", "An error has occurred: ".red(), error.to_string().red());
                        }
                    }
                }
            },
            Err(error) => {
                println!("{} {}", "An error has occurred: ".red(), error.to_string().red());
            }
        }
    }
}