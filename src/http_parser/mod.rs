use std::collections::HashMap;
use std::fmt;
use std::str::Lines;

#[derive(Debug)]
pub struct HTTPRequest {
    pub method: Method,
    pub path: String,
    pub http_version: HTTPVersion,
    pub headers: HashMap<String, String>,
    pub hostname: String,
}

struct TopLine {
    method: Method,
    path: String,
    http_version: HTTPVersion,
}

#[derive(Debug)]
pub enum Method {
    Get,
}

#[derive(Debug)]
pub enum HTTPVersion {
    OnePointOne,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidMethod,
    InvalidHTTPVersion,
    MissingTopLine,
    NoPath,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidMethod => write!(f, "Invalid HTTP method"),
            ParseError::InvalidHTTPVersion => write!(f, "Invalid HTTP version"),
            ParseError::MissingTopLine => write!(f, "Missing top line"),
            ParseError::NoPath => write!(f, "No path"),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse_request(request: &str) -> Result<HTTPRequest, ParseError> {
    let mut lines = request.lines();
    let top_line_opt = lines.next();
    if let Some(top_line) = top_line_opt {
        let parsed_top_line_opt = parse_top_line(top_line);
        if let Ok(parsed_top_line) = parsed_top_line_opt {
            let headers = parse_headers(&mut lines);
            let host = headers.get("host");
            if let Some(host) = host {
                let hostname = parse_hostname(host.to_string());
                return Ok(HTTPRequest {
                    method: parsed_top_line.method,
                    path: parsed_top_line.path,
                    http_version: parsed_top_line.http_version,
                    hostname,
                    headers,
                });
            }
        }
    }
    Err(ParseError::MissingTopLine)
}

fn parse_top_line(top_line: &str) -> Result<TopLine, ParseError> {
    let mut parts = top_line.split(" ");
    let method_opt = parts.next();
    if method_opt.is_some() {
        let parsed_method = parse_method(method_opt.unwrap());
        if let Ok(method) = parsed_method {
            let path_opt = parts.next();
            if let Some(path) = path_opt {
                let http_version_opt = parts.next();
                if let Some(http_version) = http_version_opt {
                    let parsed_http_version = parse_http_version(http_version);
                    if let Ok(http_version) = parsed_http_version {
                        Ok(TopLine {
                            method,
                            path: path.to_string(),
                            http_version,
                        })
                    } else {
                        Err(ParseError::InvalidHTTPVersion)
                    }
                } else {
                    Err(ParseError::InvalidHTTPVersion)
                }
            } else {
                Err(ParseError::NoPath)
            }
        } else {
            Err(ParseError::InvalidMethod)
        }
    } else {
        Err(ParseError::InvalidMethod)
    }
}

fn parse_method(method: &str) -> Result<Method, ParseError> {
    let lowercase_method = method.to_lowercase();

    match lowercase_method.as_str() {
        "get" => Ok(Method::Get),
        _ => Err(ParseError::InvalidMethod),
    }
}

fn parse_http_version(http_version: &str) -> Result<HTTPVersion, ParseError> {
    match http_version {
        "HTTP/1.1" => Ok(HTTPVersion::OnePointOne),
        _ => Err(ParseError::InvalidHTTPVersion),
    }
}

fn parse_headers(lines: &mut Lines) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    for line in lines.into_iter() {
        let parts_opt = line.split_once(": ");
        if let Some(parts) = parts_opt {
            let (header, value) = parts;
            headers.insert(header.to_lowercase(), value.to_string());
        }
    }
    return headers;
}

fn parse_hostname(host: String) -> String {
    let splits = host.split_once(":");
    if let Some((host, _port)) = splits {
        return host.to_string();
    }
    "".to_string()
}
