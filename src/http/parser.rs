use std::str::FromStr;
use crate::http::parser::Header::OtherHeader;
use crate::http::parser::HttpParseError::{InvalidHttpVersion, InvalidUri};
use crate::http::uri::Uri;

#[derive(Debug)]
pub enum  HttpParseError {
    InvalidUri,
    InvalidHttpMethod,
    InvalidHttpVersion,
    InvalidHeader,
    InvalidRequest,
    NonNumericContentLength
}
impl std::fmt::Display for HttpParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpParseError::InvalidUri => write!(f, "Invalid URL"),
            HttpParseError::InvalidHttpMethod => write!(f, "Invalid HTTP method"),
            HttpParseError::InvalidHttpVersion => write!(f, "Invalid HTTP version"),
            HttpParseError::InvalidHeader => write!(f, "Invalid header"),
            HttpParseError::InvalidRequest => write!(f, "Invalid request"),
            HttpParseError::NonNumericContentLength => write!(f, "Non numeric content length"),
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
    pub(crate) fn url(&self) -> Uri {
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
    pub fn get_headers(&self) -> &Vec<Header> {
        &self.headers
    }
    pub fn get_header(&self, index: usize) -> Option<&Header> {
        self.headers.get(index)
    }

    pub fn get_request_line(&self) -> &RequestLine {
        &self.request_line
    }
}

impl FromStr for HttpRequest {
    type Err = HttpParseError;

    /// Parses the header of a http request
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



        match Uri::from_str(request_headers[1]) {
            Ok(_url) => url = _url,
            Err(_) => return Err(InvalidUri),
        }

        protocol = request_headers[2].trim().to_lowercase();

        let headers = &lines[1..];
        //TODO parse headers
        let headers = parse_headers(headers)?;

        Ok(HttpRequest {
            request_line: RequestLine {
                verb,
                protocol,
                url,
            },
            headers,
        })
    }
}

fn parse_headers(headers: &[&str]) -> Result<Vec<Header>, HttpParseError> {
    let mut headers_vec = Vec::with_capacity(headers.len());
    for header in headers {
        let parts = header.split(":").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(HttpParseError::InvalidHeader);
        }
        match parts[0].trim().to_lowercase().as_str() {
            "host" => headers_vec.push(Header::Host(parts[1].trim().to_string())),
            "accept" => headers_vec.push(Header::Accept(parts[1].trim().to_string())),
            "authorization" => headers_vec.push(Header::Authorization(parts[1].trim().to_string())),
            "referer" => headers_vec.push(Header::Referer(parts[1].trim().to_string())),
            "user-agent" => headers_vec.push(Header::UserAgent(parts[1].trim().to_string())),
            "content-type" => headers_vec.push(Header::ContentType(parts[1].trim().to_string())),
            "content-length" => {
                match parts[1].trim().parse::<usize>() {
                    Ok(length) => headers_vec.push(Header::ContentLength(length)),
                    Err(_) => {return Err(HttpParseError::NonNumericContentLength)}
                }
            }
            _ => headers_vec.push(OtherHeader(parts[0].trim().to_string(), parts[1].trim().to_string()))
        }
    }
    return Ok(vec![]);
}

pub struct RequestLine{
    verb: Verb ,
    protocol: String,
    url: Uri,
}

impl RequestLine{
    pub fn verb(&self) -> &Verb{&self.verb}
    pub fn protocol(&self) -> &str{&self.protocol}
    pub fn url(&self) -> &Uri{&self.url}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Header {
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
pub enum Verb{
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
        assert_eq!(requests[0].verb(), Verb::Get);
        assert_eq!(requests[0].url(), Uri::from_str(" /api/users").unwrap());
        assert_eq!(requests[1].verb(), Verb::Get);
        assert_eq!(requests[1].protocol(), String::from("http/1.2"));
    }
}