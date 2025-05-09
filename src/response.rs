// src/response.rs
pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    /// Cria uma resposta de texto 200 OK
    pub fn text(body: &str) -> Self {
        Response { status: 200, body: body.into() }
    }

    /// Altera o status
    pub fn with_status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }

    /// Serializa em HTTP/1.1 bruto
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