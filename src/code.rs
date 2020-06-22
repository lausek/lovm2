use crate::value::CoValue;
use crate::var::Variable;

pub struct CodeObject {
    pub(crate) consts: Vec<CoValue>,
    pub(crate) locals: Vec<Variable>,
    pub(crate) globals: Vec<Variable>,
    
    pub(crate) code: Vec<u8>,
}

pub struct CodeObjectBuilder {
    consts: Vec<CoValue>,
    locals: Vec<Variable>,
    globals: Vec<Variable>,
    
    code: Option<Vec<u8>>,
}

impl CodeObjectBuilder {
    pub fn new() -> Self {
        Self {
            consts: vec![],
            locals: vec![],
            globals: vec![],
            
            code: None,
        }
    }

    pub fn consts(mut self, consts: Vec<CoValue>) -> Self {
        self.consts = consts;
        self
    }

    pub fn locals(mut self, locals: Vec<Variable>) -> Self {
        self.locals = locals;
        self
    }

    pub fn globals(mut self, globals: Vec<Variable>) -> Self {
        self.globals = globals;
        self
    }

    pub fn code(mut self, code: Vec<u8>) -> Self {
        self.code = Some(code);
        self
    }
    
    pub fn build(self) -> Result<CodeObject, String> {
        if self.code.is_none() {
            return Err("code missing".to_string());
        }
        Ok(CodeObject {
            consts: self.consts,
            locals: self.locals,
            globals: self.globals,
            
            code: self.code.unwrap(),
        })
    }
}
