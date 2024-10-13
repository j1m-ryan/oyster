use crate::http_parser;
use crate::http_parser::HTTPRequest;
use crate::response::Response;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct App {
    routes: HashMap<String, Box<dyn Fn(HTTPRequest, Response) -> Response>>,
}

impl App {
    pub fn new() -> Self {
        App {
            routes: HashMap::new(),
        }
    }
    pub fn get<F>(&mut self, path: &str, f: F)
    where
        F: Fn(HTTPRequest, Response) -> Response + 'static,
    {
        self.routes.insert(path.to_string(), Box::new(f));
    }
    pub fn listen(&self, port: u32) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        println!("Listening on port {}", port);

        for stream in listener.incoming() {
            let stream = stream?;
            self.handle_client(stream)
        }

        return Ok(());
    }
    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];

        if let Ok(_) = stream.read(&mut buffer) {
            let request = String::from_utf8_lossy(&buffer[..]);
            if let Ok(http_request) = http_parser::parse_request(&request) {
                let response = if let Some(handler) = self.routes.get(&http_request.path) {
                    let res = handler(http_request, Response::new());
                    format!(
                        "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}",
                        res.status,
                        res.text.len(),
                        res.text
                    )
                } else {
                    let res = "Not found";
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        res.len(),
                        res
                    )
                };
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
    }
}
