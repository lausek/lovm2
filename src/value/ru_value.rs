use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use lovm2_error::*;

pub type RuDict = HashMap<RuValue, RuValue>;
pub type RuDictRef = Rc<RefCell<RuDict>>;
pub type RuList = Vec<RuValue>;
pub type RuListRef = Rc<RefCell<RuList>>;
pub type RuValueRef = Rc<RefCell<RuValue>>;

pub fn box_ruvalue(value: RuValue) -> RuValueRef {
    Rc::new(RefCell::new(value))
}

/// runtime values
///
/// this layout is more suited for runtime representation than `CoValue`
#[derive(Clone, Debug, PartialEq)]
pub enum RuValue {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Dict(RuDictRef),
    List(RuListRef),
}

impl RuValue {
    pub fn delete(&self, key: RuValue) -> Lovm2Result<()> {
        match self {
            RuValue::Dict(dict) => {
                dict.borrow_mut().remove(&key);
            }
            RuValue::List(list) => {
                if let RuValue::Int(key) = key.into_integer()? {
                    list.borrow_mut().remove(key as usize);
                } else {
                    unreachable!()
                }
            }
            _ => return Err("value does not support `delete`".into()),
        }
        Ok(())
    }

    pub fn get(&self, key: RuValue) -> Lovm2Result<RuValue> {
        match self {
            RuValue::Dict(dict) => match dict.borrow().get(&key) {
                Some(val) => Ok(val.clone()),
                None => Err("key not found on value".into()),
            },
            RuValue::List(list) => {
                if let RuValue::Int(key) = key.into_integer()? {
                    match list.borrow().get(key as usize) {
                        Some(val) => Ok(val.clone()),
                        None => Err("key not found on value".into()),
                    }
                } else {
                    unreachable!()
                }
            }
            _ => Err("value does not support `get`".into()),
        }
    }

    pub fn len(&self) -> Lovm2Result<usize> {
        match self {
            RuValue::Dict(dict) => Ok(dict.borrow().len()),
            RuValue::List(list) => Ok(list.borrow().len()),
            _ => Err("value does not support `len`".into()),
        }
    }

    pub fn set(&mut self, key: RuValue, val: RuValue) -> Lovm2Result<()> {
        match self {
            RuValue::Dict(dict) => {
                dict.borrow_mut().insert(key, val);
                Ok(())
            }
            RuValue::List(list) => {
                if let RuValue::Int(idx) = key.into_integer()? {
                    let mut list = list.borrow_mut();
                    if list.len() == idx as usize {
                        list.push(val);
                    } else {
                        list[idx as usize] = val;
                    }
                    Ok(())
                } else {
                    unreachable!()
                }
            }
            _ => Err("value does not support `set`".into()),
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
            RuValue::Nil => unimplemented!(),
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
            RuValue::Dict(d) => write!(
                f,
                "{{{}}}",
                d.borrow()
                    .iter()
                    .map(|(key, val)| format!("{}: {}", key, val))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            RuValue::List(ls) => write!(
                f,
                "[{}]",
                ls.borrow()
                    .iter()
                    .map(|val| format!("{}", val))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            _ => unimplemented!(),
        }
    }
}
