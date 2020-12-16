//! wrapper type for identifiers

use serde::{Deserialize, Serialize};

/// a thin wrapper around an identifier name.
///
/// lovm2 needs a clear distinction between strings and variables. variables cannot be pushed onto
/// stack.
#[derive(Clone, Debug, Eq, Hash, Deserialize, Serialize)]
pub struct Variable(String);

impl Variable {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl From<&str> for Variable {
    fn from(name: &str) -> Self {
        Variable(name.to_string())
    }
}

impl From<String> for Variable {
    fn from(name: String) -> Self {
        Variable(name)
    }
}

impl From<&Self> for Variable {
    fn from(name: &Self) -> Self {
        name.clone()
    }
}

impl AsRef<str> for Variable {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T> PartialEq<T> for Variable
where
    T: AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        self.0 == other.as_ref()
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
