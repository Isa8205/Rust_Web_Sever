pub struct Headers {
    pub items: Vec<String>
}

impl Headers {
    /// Finds the header specified in `name` and returns its value.
    /// `None` will be returned in the event the header is not found.
    /// The matching is case sensitive so ensure to write it as it is in the `MDN` documentation
    /// 
    /// # Examples
    /// ```
    /// use rust_web_server::core::header::Headers;
    /// let headers = Headers {
    ///     items: vec!["Content-Type: text/html".to_string()]
    /// };
    /// assert_eq!(headers.get("Content-Type"), Some("text/html"));
    /// ```
    pub fn get(&self, name: &str) -> Option<&str> {
        let value = self.items
        .iter().
        find(|i| i.split(':').nth(0).unwrap()
        .trim().to_lowercase() == name.to_string());

        match value {
            Some(v) => {
                v.split(':').nth(1)
            },
            None => None
        }
    }
}