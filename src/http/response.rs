use std::collections::HashMap;
use std::fmt::Write;

pub struct HttpResponse<'a> {
    pub status_code: u16,
    pub headers: HashMap<&'a str, &'a str>,
    pub body: String
}

impl<'a> HttpResponse<'a> {
    pub fn new(status_code: u16) -> Self {
        HttpResponse {
            status_code,
            headers: HashMap::new(),
            body: String::new()
        }
    }

    pub fn add_header(mut self, key: &'a str, value: &'a str) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn set_body(mut self, body: String) -> Self {
        self.body = body;
        self
    }

    pub fn to_string(&self) -> String {
        let mut response = String::new();
        writeln!(response, "HTTP/1.1 {} {}", self.status_code, self.status_message()).unwrap();
        for (key, value) in &self.headers {
            writeln!(response, "{}: {}", key, value).unwrap();
        }
        writeln!(response, "\r\n{}", self.body).unwrap();
        response
    }

    fn status_message(&self) -> &str {
        match self.status_code {
            200 => "OK",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Unknown Status",
        }
    }
}