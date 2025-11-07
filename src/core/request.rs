/// The structured request from the client.
#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    headers: Vec<String>,
    body: Option<String>,
}

impl Request {
    pub fn new(method: String, path: String, headers: Vec<String>) -> Request {
        Request {
            method,
            path,
            headers,
            body: None,
        }
    }
    pub fn get_headers(&self) -> &Vec<String> {
        &self.headers
    }
}
