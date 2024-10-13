#[derive(Clone)]
pub struct Response {
    pub text: String,
    pub status: u16,
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
