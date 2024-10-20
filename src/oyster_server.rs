use crate::http_manager::HTTPManager;

pub struct Oyster {
    pub http: HTTPManager,
}

impl Oyster {
    pub fn new() -> Self {
        Oyster {
            http: HTTPManager::new(),
        }
    }
    pub fn start(&self) {
        self.http.start();
    }
}

