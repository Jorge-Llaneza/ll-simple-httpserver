use std::str::FromStr;
use crate::http::parser::HttpParseError::{InvalidHttpVersion, InvalidUrl};

#[derive(Debug)]
pub enum  HttpParseError {
    InvalidUrl,
    InvalidHttpMethod,
    InvalidHttpVersion,
    InvalidHeader,
    InvalidRequest,
}
impl std::fmt::Display for HttpParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpParseError::InvalidUrl => write!(f, "Invalid URL"),
            HttpParseError::InvalidHttpMethod => write!(f, "Invalid HTTP method"),
            HttpParseError::InvalidHttpVersion => write!(f, "Invalid HTTP version"),
            HttpParseError::InvalidHeader => write!(f, "Invalid header"),
            HttpParseError::InvalidRequest => write!(f, "Invalid request"),
        }
    }
}

impl std::error::Error for HttpParseError {}

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    pub tokens: Vec<String>

}

pub struct HttpRequest{
    request_line: RequestLine,
    headers: Vec<Header>,
}

impl HttpRequest {
    pub(crate) fn verb(&self) -> Verb {
        self.request_line.verb.clone()
    }
    pub(crate) fn url(&self) -> Url {
        self.request_line.url.clone()
    }
    pub(crate) fn protocol(&self) -> String {
        self.request_line.protocol.clone()
    }
}

impl HttpRequest{
    pub fn new(request_line: RequestLine, headers: Vec<Header>) -> HttpRequest{
        HttpRequest{
            request_line,
            headers
        }
    }
}

impl FromStr for HttpRequest {
    type Err = HttpParseError;
    fn from_str(s: &str) -> Result<HttpRequest, Self::Err> {
        let verb;
        let url;
        let protocol;

        let lines = s.split("\r\n").collect::<Vec<&str>>();
        if lines.len() == 0 {
            return Err(HttpParseError::InvalidRequest);
        }
        let request_headers = lines[0].split(" ").collect::<Vec<&str>>();
        if request_headers.len() != 3 {
            return Err(HttpParseError::InvalidRequest);
        }

        verb = Verb::from_str(request_headers[0])?;


        Url::
        match Url::from_str(request_headers[1]) {
            Ok(_url) => url = _url,
            Err(_) => return Err(InvalidUrl),
        }

        protocol = request_headers[2].to_string();

        let headers = &lines[1..];
        //TODO parse headers

        Ok(HttpRequest {
            request_line: RequestLine {
                verb,
                protocol,
                url,
            },
            headers: vec![],
        })
    }
}


struct RequestLine{
    verb: Verb ,
    protocol: String,
    url: Url,
}

#[derive(Debug, Clone, PartialEq)]
enum Header {
    Host(String),
    Accept(String),
    Authorization(String),
    Referer(String),
    UserAgent(String),
    ContentLength(usize),
    ContentType(String),
    OtherHeader(String, String),
}
#[derive(Debug, Clone, PartialEq)]
enum Verb{
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl Verb{
    pub fn to_string(&self) -> String {
        match self {
            &Verb::Get => String::from("GET"),
            &Verb::Post => String::from("POST"),
            &Verb::Put => String::from("PUT"),
            &Verb::Patch => String::from("PATCH"),
            &Verb::Delete => String::from("DELETE"),
        }
    }
    pub fn from_str(s: &str) -> Result<Verb, HttpParseError> {
        match s.to_lowercase().trim() {
            "get" => Ok(Verb::Get),
            "post" => Ok(Verb::Post),
            "put" => Ok(Verb::Put),
            "patch" => Ok(Verb::Patch),
            "delete" => Ok(Verb::Delete),
            _ => Err(InvalidHttpVersion),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_only_request_line_httprequest() {
        let requests = vec![
            HttpRequest::from_str("gEt /api/users HTtP/1.1").unwrap(),
            HttpRequest::from_str("gEt /api/gigis HTtP/1.2").unwrap(),
            HttpRequest::from_str("gEt /api/users HTtP/1.3").unwrap(),
            HttpRequest::from_str("gEt / HTtP/1.4").unwrap(),
            HttpRequest::from_str("gEt //users HTtP/1.5").unwrap(),
        ];
        assert_eq!(requests[0].verb(), Verb::Get)

    }
}