use lovm2::prelude::*;
use lovm2_extend::prelude::*;

use std::collections::HashMap;

#[lovm2_object]
pub struct Buffer {
    pub inner: Vec<u8>,
}

#[lovm2_object]
pub struct File {
    pub inner: std::fs::File,
}

#[lovm2_object]
pub struct Regex {
    pub inner: regex::Regex,
}

pub enum Method {
    POST,
    GET,
    DELETE,
    PUT,
}

#[lovm2_object]
pub struct Request {
    pub url: Option<String>,
    pub headers: HashMap<String, String>,
    pub method: Method,
}

#[lovm2_object]
pub struct Response {
    pub status: i64,
    pub body: Vec<u8>,
}
