use crate::HTTPServer;

pub struct HTTPManager {
    servers: Vec<HTTPServer>,
}

impl HTTPManager {
    pub fn new() -> Self {
        HTTPManager {
            servers: Vec::new(),
        }
    }

    pub fn start(&self) {
        let mut handles = Vec::new();

        for server in &self.servers {
            let handle = server.start();
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
    pub fn add_server(&mut self, server: HTTPServer) {
        self.servers.push(server);
    }
}
