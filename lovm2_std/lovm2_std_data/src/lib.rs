use lovm2::prelude::*;
use lovm2_extend::prelude::*;

use std::collections::HashMap;

// buffer
#[lovm2_object]
#[derive(Default)]
pub struct Buffer {
    pub inner: Vec<u8>,
    pub roff: usize,
}

// fs
#[lovm2_object]
pub struct File {
    pub inner: std::fs::File,
}

// regex
#[lovm2_object]
pub struct Regex {
    pub inner: regex::Regex,
}

// net
#[lovm2_object]
pub struct Request {
    pub url: String,
    pub headers: HashMap<String, String>,
    pub method: Method,
    pub body: Option<Vec<u8>>,
}

#[lovm2_object]
pub struct Response {
    pub status: i64,
    pub body: Vec<u8>,
}

#[doc(hidden)]
pub enum Method {
    Delete,
    Get,
    Post,
    Put,
}

impl std::convert::TryFrom<String> for Method {
    type Error = Lovm2Error;

    fn try_from(mut from: String) -> Result<Self, Self::Error> {
        from.make_ascii_lowercase();
        match from.as_ref() {
            "delete" => Ok(Self::Delete),
            "get" => Ok(Self::Get),
            "post" => Ok(Self::Post),
            "put" => Ok(Self::Put),
            _ => Err(Lovm2Error::from("not a valid request method")),
        }
    }
}
