use super::request::*;
use super::host::HostDescription;

#[derive(Debug)]
pub struct HttpResponse;// TODO

pub type RequestHandler = fn(request: HttpRequest) -> HttpResponse;

#[derive(Debug)]
pub struct HttpRequestTemplate {
    http_verb: HttpVerb,
    path: UrlPath,
    host: HostDescription,
    params: Option<UrlParams>,
    content: Option<String>,
    answer: RequestHandler,
}

impl HttpRequestTemplate {
    pub fn new(http_verb: HttpVerb,
               host: HostDescription,
               path: UrlPath,
               params: Option<UrlParams>,
               content: Option<String>,
               answer: RequestHandler) -> HttpRequestTemplate {
        HttpRequestTemplate { http_verb, host, path, params, content, answer }
    }

    pub fn response(&self, request: HttpRequest) -> HttpResponse {
        (self.answer)(request)
    }
}