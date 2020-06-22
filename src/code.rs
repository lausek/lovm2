use crate::value::CoValue;
use crate::var::Variable;

pub struct CodeObject {
    pub(crate) consts: Vec<CoValue>,
    pub(crate) locals: Vec<Variable>,
    pub(crate) globals: Vec<Variable>,
    
    pub(crate) code: Vec<u8>,
}
