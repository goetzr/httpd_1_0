use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;

const CRLF: &'static str = "\r\n";
const HTTP_VERSION_TAG: &'static str = "HTTP";

pub struct Request {
    method: HttpMethod,
    uri: String,
    version: HttpVersion,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

pub enum HttpMethod {
    Get,
    Head,
    Post,
}

pub struct HttpVersion {
    major: u8,
    minor: u8,
}

#[derive(Debug)]
enum ParseRequestError {
    RequestLine,
    UnsupportedMethod(String),
    UnsupportedVersion,
}

enum ParseRequestLineError {
    
}

#[derive(Debug)]
struct ParseHttpVersionError;

impl fmt::Display for ParseHttpRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "malformed HTTP request")
    }
}

impl std::error::Error for ParseHttpRequestError {}

impl FromStr for Request {
    type Err = ParseHttpRequestError;

    // TODO: Make errors much more course and log details to log file?

    fn from_str(s: &str) -> Result<Self, Self::Err> { 
        let mut lines = s.split(CRLF);
        if let Some(request_line) = lines.next() {
            
        }
        Err(ParseHttpRequestError)
    }
}

// TODO: Need to be able to return error indicating unsupported method / HTTP version
fn parse_request_line(line: &str) -> Result<(HttpMethod, String, HttpVersion), ParseHttpRequestError> {
    const METHODS: [&str; 3] = ["get", "head", "post"];
    let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
    if parts.len() >= 2 {
        let method = parts[0].to_ascii_lowercase();
        if 
    }
    Err(ParseHttpRequestError)
}

impl fmt::Display for ParseHttpVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "malformed HTTP version")
    }
}

impl std::error::Error for ParseHttpVersionError {}

impl FromStr for HttpVersion {
    type Err = ParseHttpVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("/").collect::<Vec<_>>();
        if parts.len() == 2 && parts[0] == HTTP_VERSION_TAG {
            let numbers = parts[1].split(".").collect::<Vec<_>>();
            if numbers.len() == 2 {
                let major = numbers[0].parse::<u8>().map_err(|_| ParseHttpVersionError)?;
                let minor = numbers[1].parse::<u8>().map_err(|_| ParseHttpVersionError)?;
                return Ok( HttpVersion { major, minor } );
            }  
        }
        Err(ParseHttpVersionError)
    }
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}