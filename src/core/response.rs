pub struct Response {
    pub status: u8,
    headers: Option<Vec<String>>,
    pub body: Option<String>,
}

impl Response {
    /// Instantiates and returns a new Response
    ///
    /// # Examples
    ///
    pub fn new(status: u8, body: Option<String>) -> Response {
        Response {
            status,
            headers: None,
            body,
        }
    }

    /// Sets the status of the response and returns the new status
    ///
    /// # Examples
    ///
    pub fn set_status(&mut self, value: u8) -> u8 {
        self.status = value;
        self.status
    }

    /// Adds a single header to the response
    ///
    /// # Examples
    ///
    pub fn set_headers(&mut self, header: String, value: &str) {
        let full_header = format!("{header}: {value}");
        match self.headers.as_mut() {
            Some(headers) => headers.push(full_header),
            None => self.headers = Some(vec![full_header]),
        }
    }

    /// Sets the body of the response
    ///
    /// # Examples
    ///
    pub fn send<T>(&mut self, content: T)
    where
        T: Into<String>,
    {
        self.body = Some(content.into());
    }
}
