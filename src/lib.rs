use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Clone)]
pub struct Request {
    pub host: String,
}

impl Request {
    pub fn new() -> Self {
        Request {
            host: "".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Response {
    text: String,
    status: u16,
}

impl Response {
    pub fn new() -> Self {
        Response {
            text: "".to_string(),
            status: 200,
        }
    }
    pub fn send(&mut self, response: &str) -> Response {
        self.text = response.to_string();
        self.clone()
    }
    pub fn status(&mut self, status: u16) -> Response {
        self.status = status;
        self.clone()
    }
}

pub struct App {
    routes: HashMap<String, Box<dyn Fn(Request, Response) -> Response>>,
}

impl App {
    pub fn new() -> Self {
        App {
            routes: HashMap::new(),
        }
    }
    pub fn get<F>(&mut self, path: &str, f: F)
    where
        F: Fn(Request, Response) -> Response + 'static,
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
            let path = self.parse_path(&request);
            let host = self.parse_host(&request);
            let mut req = Request::new();
            req.host = host;
            let response = if let Some(handler) = self.routes.get(&path) {
                let res = handler(req, Response::new());
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

    fn parse_path(&self, request: &str) -> String {
        if let Some(first_line) = request.lines().next() {
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() >= 2 {
                return parts[1].to_string();
            }
        }
        return "/".to_string();
    }

    fn parse_host(&self, request: &str) -> String {
        for line in request.lines() {
            if line.starts_with("Host") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return parts[1].to_string();
                }
            }
        }
        return "".to_string();
    }
}
