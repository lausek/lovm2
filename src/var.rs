#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Variable(String);

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

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
