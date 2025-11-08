
#[derive(Debug)]
pub struct QueryParams {
    pub items: Vec<(String, String)>
}

impl QueryParams {
    pub fn new() -> QueryParams {
        QueryParams {
            items: Vec::new()
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.items
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    pub fn get_all(&self) -> &Vec<(String, String)> {
        &self.items
    }
}

/// The structured request from the client.
#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query_params: QueryParams,
    headers: Vec<String>,
    body: Option<String>,
}

impl Request {
    pub fn new(method: String, path: String, query_params: QueryParams, headers: Vec<String>) -> Request {
        Request {
            method,
            path,
            query_params,
            headers,
            body: None,
        }
    }
    pub fn get_headers(&self) -> &Vec<String> {
        &self.headers
    }
}
