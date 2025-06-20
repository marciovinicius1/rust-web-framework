// src/response.rs
pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    
    pub fn text(body: &String) -> Self {
        Response { status: 200, body: body.into() }
    }
    
    pub fn with_status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }
    
    pub fn to_http(&self) -> String {
        format!(
            "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\
             Content-Type: text/plain\r\n\r\n{}",
            self.status,
            self.body.len(),
            self.body
        )
    }
}