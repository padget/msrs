
use super::request::{HttpRequest, HttpVerb, UrlParams, UrlPath};
use super::template::{HttpRequestTemplate, HttpResponse, RequestHandler};
use std::option::Option;
use std::string::String;
use super::host::{HostDescription, IpAddress, Port};
use std::marker::PhantomData;
use std;

/// Region Macros
macro_rules! str {
    ($x:expr) => {std::string::String::from($x)};
}

macro_rules! map {
    ($($key:expr => $value:expr),+ ) => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(m.insert($key, $value);)+
            m
        }
     };
}

mod marker {
    pub struct FirstStep;

    pub struct NextStep;

    pub struct ToGoal;

    pub struct OnGoal;
}


/// Public functions/structures
pub fn to(ip: IpAddress, port: Port) -> DslHttpVerbStep<marker::ToGoal> {
    DslHttpVerbStep {
        host: HostDescription::new(ip, port),
        _gm: PhantomData,
    }
}

pub fn on() -> DslHttpVerbStep<marker::OnGoal> {
    DslHttpVerbStep {
        host: HostDescription::default(),
        _gm: PhantomData,
    }
}

pub struct DslHttpVerbStep<GoalMarker> {
    host: HostDescription,

    _gm: PhantomData<GoalMarker>,
}

impl<GoalMarker> DslHttpVerbStep<GoalMarker> {
    #[allow(dead_code)]
    fn new(host: HostDescription) -> DslHttpVerbStep<GoalMarker> {
        DslHttpVerbStep { host, _gm: PhantomData }
    }
}

pub struct DslPathStep<GoalMarker, StepMarker> {
    host: HostDescription,
    http_verb: HttpVerb,
    path: UrlPath,

    _sm: PhantomData<StepMarker>,
    _gm: PhantomData<GoalMarker>,
}

impl<GoalMarker, StepMarker> DslPathStep<GoalMarker, StepMarker> {
    fn new(host: HostDescription, http_verb: HttpVerb, path: UrlPath) -> DslPathStep<GoalMarker, StepMarker> {
        DslPathStep { host, path, http_verb, _gm: PhantomData, _sm: PhantomData }
    }
}

pub struct DslUrlParamsStep<GoalMarker> {
    host: HostDescription,
    path: UrlPath,
    http_verb: HttpVerb,
    params: Option<UrlParams>,

    _gm: PhantomData<GoalMarker>,
}

impl<GoalMarker> DslUrlParamsStep<GoalMarker> {
    fn new(host: HostDescription, http_verb: HttpVerb,
           path: UrlPath, params: Option<UrlParams>)
           -> DslUrlParamsStep<GoalMarker> {
        DslUrlParamsStep { host, path, http_verb, params, _gm: PhantomData }
    }
}

pub struct DslRequestStep<GoalMarker> {
    host: HostDescription,
    path: UrlPath,
    http_verb: HttpVerb,
    params: Option<UrlParams>,
    content: Option<String>,

    _gm: PhantomData<GoalMarker>,
}

impl<GoalMarker> DslRequestStep<GoalMarker> {
    fn new(host: HostDescription, http_verb: HttpVerb, path: UrlPath,
           params: Option<UrlParams>, content: Option<String>)
           -> DslRequestStep<GoalMarker> {
        DslRequestStep { host, path, http_verb, params, content, _gm: PhantomData }
    }
}


impl<GoalMarker> DslHttpVerbStep<GoalMarker> {
    pub fn get(self) -> DslPathStep<GoalMarker, marker::FirstStep> {
        DslPathStep::new(self.host, HttpVerb::GET, Vec::new())
    }

    #[allow(dead_code)]
    pub fn delete(self) -> DslPathStep<GoalMarker, marker::FirstStep> {
        DslPathStep::new(self.host, HttpVerb::DELETE, Vec::new())
    }

    #[allow(dead_code)]
    pub fn post(self) -> DslPathStep<GoalMarker, marker::FirstStep> {
        DslPathStep::new(self.host, HttpVerb::POST, Vec::new())
    }

    #[allow(dead_code)]
    pub fn put(self) -> DslPathStep<GoalMarker, marker::FirstStep> {
        DslPathStep::new(self.host, HttpVerb::PUT, Vec::new())
    }

    #[allow(dead_code)]
    pub fn patch(self) -> DslPathStep<GoalMarker, marker::FirstStep> {
        DslPathStep::new(self.host, HttpVerb::PATCH, Vec::new())
    }
}

impl<GoalMarker, StepMarker> DslPathStep<GoalMarker, StepMarker> {
    pub fn slash(self, path: &str) -> DslPathStep<GoalMarker, marker::NextStep> {
        DslPathStep::new(self.host, self.http_verb, merge_paths(self.path, path))
    }
}

impl<GoalMarker> DslPathStep<GoalMarker, marker::NextStep> {
    pub fn param(self, name: &str, value: &str) -> DslUrlParamsStep<GoalMarker> {
        DslUrlParamsStep::new(self.host, self.http_verb,self.path,
                              Some(map!(str!(name) => str!(value))))
    }
}

impl DslPathStep<marker::ToGoal, marker::NextStep> {
    pub fn request(self) -> HttpRequest {
        HttpRequest::new(self.http_verb, self.host, self.path, None, None)
    }
}

impl DslPathStep<marker::OnGoal, marker::NextStep> {
    pub fn answer(self, answer: RequestHandler) -> HttpRequestTemplate {
        HttpRequestTemplate::new(self.http_verb, self.host, self.path, None, None, answer)
    }
}


impl<GoalMarker> DslUrlParamsStep<GoalMarker> {
    pub fn param(self, name: &str, value: &str) -> DslUrlParamsStep<GoalMarker> {
        DslUrlParamsStep::new(self.host, self.http_verb, self.path,
                              merge_params(self.params, name, value))
    }

    pub fn content(self, content: &str) -> DslRequestStep<GoalMarker> {
        DslRequestStep::new(self.host, self.http_verb, self.path,
                            self.params, Some(str!(content)))
    }
}

impl DslUrlParamsStep<marker::ToGoal> {
    #[allow(dead_code)]
    pub fn request(self) -> HttpRequest {
        HttpRequest::new(self.http_verb, self.host, self.path, self.params, None)
    }
}

impl DslUrlParamsStep<marker::OnGoal> {
    #[allow(dead_code)]
    pub fn answer(self, answer: fn(request: HttpRequest) -> HttpResponse) -> HttpRequestTemplate {
        HttpRequestTemplate::new(self.http_verb, self.host, self.path, self.params, None, answer)
    }
}

impl DslRequestStep<marker::ToGoal> {
    pub fn request(self) -> HttpRequest {
        HttpRequest::new(self.http_verb, self.host, self.path, self.params, self.content)
    }
}

impl DslRequestStep<marker::OnGoal> {
    #[allow(dead_code)]
    pub fn answer(self, answer: fn(request: HttpRequest) -> HttpResponse) -> HttpRequestTemplate {
        HttpRequestTemplate::new(self.http_verb, self.host, self.path, self.params, self.content, answer)
    }
}


/// Private functions
fn merge_params(params: Option<UrlParams>, name: &str, value: &str) -> Option<UrlParams> {
    match params {
        Some(mut map) => {
            map.insert(str!(name), str!(value));
            Some(map)
        }
        None => Some(map!(str ! (name) => str ! (value)))
    }
}

fn merge_paths(paths: UrlPath, path: &str) -> UrlPath {
    let mut paths = paths;
    paths.push(str!(path));
    paths
}

