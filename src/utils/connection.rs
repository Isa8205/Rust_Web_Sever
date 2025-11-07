use std::{io::{BufReader, BufRead, Read}, net::TcpStream, sync::Arc};

use crate::core::{request::Request, response::Response};

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

    // Take the remaining lines untill the CRLF and collect them to a headers vector
    let raw_headers = request
        .lines()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()[1..]
        .to_vec();

    // Build the response and request to be passed onto the route handler
    let req = Request::new(method.unwrap(), path.unwrap(), raw_headers);

    let res = Response::new(200, None);

    return (req, res);
}
