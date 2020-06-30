use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type RuDict = HashMap<RuValue, RuValue>;
pub type RuDictRef = Rc<RefCell<RuDict>>;
pub type RuList = Vec<RuValue>;
pub type RuListRef = Rc<RefCell<RuList>>;
pub type RuValueRef = Rc<RefCell<RuValue>>;

pub fn box_ruvalue(value: RuValue) -> RuValueRef {
    Rc::new(RefCell::new(value))
}

#[derive(Clone, Debug, PartialEq)]
pub enum RuValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Dict(RuDictRef),
    List(RuListRef),
}

impl RuValue {
    pub fn get(&self, key: RuValue) -> Result<RuValue, String> {
        match self {
            RuValue::Dict(dict) => match dict.borrow().get(&key) {
                Some(val) => Ok(val.clone()),
                None => Err("key not found on value".to_string()),
            },
            RuValue::List(list) => {
                if let RuValue::Int(key) = key.to_integer()? {
                    match list.borrow().get(key as usize) {
                        Some(val) => Ok(val.clone()),
                        None => Err("key not found on value".to_string()),
                    }
                } else {
                    unreachable!()
                }
            }
            _ => Err("value does not support `get`".to_string()),
        }
    }

    pub fn len(&self) -> Result<usize, String> {
        match self {
            RuValue::Dict(dict) => Ok(dict.borrow().len()),
            RuValue::List(list) => Ok(list.borrow().len()),
            _ => Err("value does not support `len`".to_string()),
        }
    }

    pub fn set(&mut self, key: RuValue, val: RuValue) -> Result<(), String> {
        match self {
            RuValue::Dict(dict) => {
                dict.borrow_mut().insert(key, val);
                Ok(())
            }
            /*
            * TODO: implement
            RuValue::List(list) => {
            let idx = key.to_int();
            list.insert(key, val);
            Ok(())
            }
            */
            _ => Err("value does not support `set`".to_string()),
        }
    }
}

impl std::cmp::Eq for RuValue {}

impl std::hash::Hash for RuValue {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: std::hash::Hasher,
    {
        match self {
            RuValue::Bool(b) => hasher.write_u8(*b as u8),
            RuValue::Int(n) => hasher.write_i64(*n),
            RuValue::Float(_) => unimplemented!(),
            RuValue::Str(s) => hasher.write(s.as_bytes()),
            RuValue::Dict(_) => unimplemented!(),
            RuValue::List(_) => unimplemented!(),
        }
    }
}

impl std::fmt::Display for RuValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuValue::Bool(b) => write!(f, "{}", b),
            RuValue::Int(n) => write!(f, "{}", n),
            RuValue::Float(n) => write!(f, "{}", n),
            RuValue::Str(s) => write!(f, "{}", s),
            _ => unimplemented!(),
            // RuValue::Dict(RuDictRef),
            // RuValue::List(RuListRef),
        }
    }
}
