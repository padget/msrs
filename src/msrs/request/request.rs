#[allow(dead_code)]
use super::host::HostDescription;
use std::option::Option;
use std::collections::HashMap;
use std::vec::Vec;
use std::string::String;

#[derive(Debug)]
pub struct HttpRequest {
    http_verb: HttpVerb,
    path: UrlPath,
    host: HostDescription,
    params: Option<UrlParams>,
    content: Option<String>,
}

#[derive(Debug)]
pub enum HttpVerb {
    POST,
    GET,
    PUT,
    PATCH,
    DELETE,
}

pub type UrlParams = HashMap<String, String>;
pub type UrlPath = Vec<String>;

impl HttpRequest {
    pub fn new(http_verb: HttpVerb,
               host: HostDescription,
               path: UrlPath,
               params: Option<UrlParams>,
               content: Option<String>) -> HttpRequest {
        HttpRequest { http_verb, host, path, params, content }
    }
}
