use std::{io::{BufRead, BufReader, Read}, net::TcpStream, sync::Arc};

use crate::http::{request::{QueryParams, Request}, response::Response};

pub fn handle_connection(stream: Arc<TcpStream>) -> (Request, Response) {
    let mut buff_reader = BufReader::new(stream.as_ref());
    let mut request = String::new();

    // Read until double newline (end of headers)
    loop {
        let mut line = String::new();
        let bytes_read = buff_reader.read_line(&mut line).unwrap();
        if bytes_read == 0 || line == "\r\n" {
            break;
        }
        request.push_str(&line);
    }

    // If you want to support POST/PUT with body
    let content_length = request
        .lines()
        .find(|line| line.to_lowercase().starts_with("content-length:"))
        .and_then(|line| line.split(':').nth(1))
        .and_then(|len| len.trim().parse::<usize>().ok())
        .unwrap_or(0);

    let mut body = vec![0; content_length];
    if content_length > 0 {
        buff_reader.read_exact(&mut body).unwrap();
        request.push_str("\r\n");
        request.push_str(&String::from_utf8_lossy(&body));
    }

    // Take the first line of the request and convert to a vector
    let method_path_arr = request
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .collect::<Vec<&str>>();

    let method = method_path_arr.get(0).map(|s| s.to_string());
    let path = method_path_arr.get(1).map(|s| s.to_string());

    let parsed_uri = parse_uri(path.unwrap().as_str()).unwrap();
    let path = parsed_uri.0;
    let query = parsed_uri.1;

    // Take the remaining lines untill the CRLF and collect them to a headers vector
    let raw_headers = request
        .lines()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()[1..]
        .to_vec();

    // Build the response and request to be passed onto the route handler
    let req = Request::new(method.unwrap(), path, query, raw_headers);
    let res = Response::new(200, None);

    return (req, res);
}

/// Takes the uri on the request and gets the path and query params
/// If it is malformed it will return an error. 
///  
fn parse_uri(uri: &str) -> Result<(String, QueryParams), &str> {
    if uri.contains("?") {
        let path;
        let query;
        let path_parts = uri.split_once('?');

        if let Some(parts) = path_parts {
            path = parts.0;
            query = parts.1;
        } else {
            return Err("Failed to parse the uri");
        }
        
        let query_params;
        if !query.is_empty() {
            let params_vec = query.split('&')
            .map(|item| {
                let mut item = item.split('=');
                (item.next().unwrap().to_string(), item.next().unwrap().to_string())
            }).collect::<Vec<(String, String)>>();

            query_params = QueryParams {
                items: params_vec
            };
        } else {
            query_params = QueryParams::new();
        }
        Ok((path.to_string(), query_params))

    } else {
        Ok((uri.to_string(), QueryParams::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uri_with_query() {
        let (path, query) = parse_uri("/api/users?name=John").unwrap();
        assert_eq!(("/api/users", &vec![("name".to_string(), "John".to_string())]), (path.as_str(), query.get_all()));
        assert_eq!(("/api/users", &"John".to_string()), (path.as_str(), query.get("name").unwrap()));
    }

    #[test]
    fn test_parse_uri_no_query() {
        let (path, query) = parse_uri("/api/users").unwrap();
        assert_eq!(("/api/users", &Vec::new()), (path.as_str(), query.get_all()));
        assert_eq!(("/api/users", None), (path.as_str(), query.get("")));
    }
    
    #[test]
    fn test_parse_uri_empty_query() {
        let (path, query) = parse_uri("/api/users?").unwrap();
        assert_eq!(("/api/users", &Vec::new()), (path.as_str(), query.get_all()));
        assert_eq!(("/api/users", None), (path.as_str(), query.get("")));
    }
}