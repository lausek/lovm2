//! Wrapper type for identifiers

use serde::{Deserialize, Serialize};

/// A thin wrapper around an identifier name.
///
/// `lovm2` needs a clear distinction between strings and variables. Variables cannot be pushed onto
/// stack.
#[derive(Clone, Debug, Eq, Hash, Deserialize, Serialize)]
pub struct LV2Variable(String);

impl LV2Variable {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl From<&str> for LV2Variable {
    fn from(name: &str) -> Self {
        LV2Variable(name.to_string())
    }
}

impl From<String> for LV2Variable {
    fn from(name: String) -> Self {
        LV2Variable(name)
    }
}

impl From<&Self> for LV2Variable {
    fn from(name: &Self) -> Self {
        name.clone()
    }
}

impl AsRef<str> for LV2Variable {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T> PartialEq<T> for LV2Variable
where
    T: AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        self.0 == other.as_ref()
    }
}

impl std::fmt::Display for LV2Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
