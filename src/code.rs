use crate::value::CoValue;
use crate::var::Variable;

pub struct CodeObject {
    consts: Vec<CoValue>,
    locals: Vec<Variable>,
    globals: Vec<Variable>,
    
    code: Vec<u8>,
}
