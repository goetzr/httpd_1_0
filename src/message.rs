use std::str::FromStr;

use url::Url;

// GET /rfc/rfc1945 HTTP/1.1
pub const HTTP_VERSION_TAG: &'static str = "HTTP";

pub enum HttpMethod {
    Get,
    Head,
    Post,
}

pub struct HttpVersion {
    major: u8,
    minor: u8,
}

impl FromStr for HttpVersion {
    // TODO: Error should be unit structe
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("/").collect::<Vec<_>>();
        // TODO: Check parts.len == 2
        match parts[0] {
            HTTP_VERSION_TAG => {
                let (major, minor) = parts[1].split(".").collect::<(_, _)>();
                let version = HttpVersion { major: 1, minor: 0 };
                Ok(version)
            }
            _ => Err(format!("HTTP version must start with '{}'", HTTP_VERSION_TAG)),
        }
}

pub struct Request {
    method: HttpMethod,
    uri: String,
    version: HttpVersion
}

pub fn parse_request_line(line: &str) -> (HttpMethod, )