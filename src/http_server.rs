use crate::http_parser::HTTPRequest;
use crate::response::Response;
use crate::thread_pool::ThreadPool;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

pub struct HTTPServer {
    pub routes:
        Arc<Mutex<HashMap<String, Box<dyn Fn(HTTPRequest, Response) -> Response + Send + Sync>>>>,
    pub http_port: u16,
    pub server_name: String,
}

impl HTTPServer {
    pub fn new(http_port: u16, server_name: &str) -> Self {
        HTTPServer {
            http_port,
            server_name: server_name.to_string(),
            routes: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn default() -> Self {
        HTTPServer {
            http_port: 80,
            server_name: "".to_string(),
            routes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get<F>(&mut self, path: &str, f: F)
    where
        F: Fn(HTTPRequest, Response) -> Response + Send + Sync + 'static,
    {
        let mut routes = self.routes.lock().unwrap();
        routes.insert(path.to_string(), Box::new(f));
    }

    pub fn start(&self, thread_pool: Arc<ThreadPool>) -> std::thread::JoinHandle<()> {
        let port = self.http_port;
        let routes = Arc::clone(&self.routes);
        let server_name = self.server_name.clone();

        let handle = std::thread::spawn(move || {
            let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
            println!("Server '{}' listening on port {}", server_name, port);

            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let routes = Arc::clone(&routes);
                let server_name = server_name.clone();
                let thread_pool = Arc::clone(&thread_pool);

                let task = move || {
                    handle_client(stream, routes, server_name);
                };

                thread_pool.execute(task);
            }
        });

        handle
    }
}

fn handle_client(
    mut stream: TcpStream,
    routes: Arc<
        Mutex<HashMap<String, Box<dyn Fn(HTTPRequest, Response) -> Response + Send + Sync>>>,
    >,
    server_name: String,
) {
    let mut buffer = [0; 1024];

    if let Ok(_) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..]);
        if let Ok(http_request) = crate::http_parser::parse_request(&request) {
            if http_request.hostname == server_name {
                let path = &http_request.path;
                let routes = routes.lock().unwrap();
                let response = if let Some(handler) = routes.get(path) {
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
                        "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",
                        res.len(),
                        res
                    )
                };
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            } else {
                let res = "Server not found";
                let response = format!(
                    "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",
                    res.len(),
                    res
                );
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        } else {
            let res = "Bad Request";
            let response = format!(
                "HTTP/1.1 400 Bad Request\r\nContent-Length: {}\r\n\r\n{}",
                res.len(),
                res
            );
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
