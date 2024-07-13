use std::collections::HashMap;

pub struct HttpRequest<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub headers: HashMap<&'a str, &'a str>,
    pub body: String
}

impl<'a> HttpRequest<'a> {
    pub fn new(request: &'a str) -> Self {
        let lines: Vec<&str> = request.lines().collect();
        let (method, path) = Self::parse_request_line(lines[0]);
        let headers = Self::parse_headers(&lines[1..]);
        let body = Self::parse_body(&lines);

        HttpRequest {
            method,
            path,
            headers,
            body
        }
    }

    fn parse_request_line(line: &str) -> (&str, &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        (parts[0], parts[1])
    }
    fn parse_headers(lines: &[&'a str]) -> HashMap<&'a str, &'a str> {
        let mut headers = HashMap::new();
        for line in lines.iter() {
            if line.is_empty() {
                break;
            }

            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                headers.insert(parts[0].trim(), parts[1].trim());
            }
        }
        headers
    }

    fn parse_body(lines: &[&str]) -> String {
        let mut body_start = false;
        let mut body_lines = Vec::new();
        for line in lines.iter() {
            if body_start {
                body_lines.push(*line);
            }
            if line.is_empty() {
                body_start = true;
            }
        }
        body_lines.join("\n")
    }
}