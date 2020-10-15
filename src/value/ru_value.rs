use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use lovm2_error::*;

use crate::value::{instantiate, CoValue};

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
/*
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
*/

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum RuValue {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Dict(HashMap<RuValue, RuValue>),
    List(Vec<RuValue>),
    #[serde(serialize_with = "serialize_ruvalue_ref")]
    #[serde(deserialize_with = "deserialize_ruvalue_ref")]
    Ref(Option<RuValueRef>),
}

impl RuValue {
    pub fn delete(&mut self, key: RuValue) -> Lovm2Result<()> {
        match self {
            RuValue::Dict(dict) => {
                dict.remove(&key);
            }
            RuValue::List(list) => {
                if let RuValue::Int(key) = key.into_integer()? {
                    list.remove(key as usize);
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
            RuValue::Dict(dict) => match dict.get(&key) {
                Some(val) => Ok(val.clone()),
                None => Err(format!("key `{}` not found on value", key).into()),
            },
            RuValue::List(list) => {
                if let RuValue::Int(key) = key.into_integer()? {
                    match list.get(key as usize) {
                        Some(val) => Ok(val.clone()),
                        None => Err(format!("key `{}` not found on value", key).into()),
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
            RuValue::Dict(dict) => Ok(dict.len()),
            RuValue::List(list) => Ok(list.len()),
            _ => Err("value does not support `len`".into()),
        }
    }

    pub fn set(&mut self, key: RuValue, val: RuValue) -> Lovm2Result<()> {
        match self {
            RuValue::Dict(dict) => {
                dict.insert(key, val);
                Ok(())
            }
            RuValue::List(list) => {
                if let RuValue::Int(idx) = key.into_integer()? {
                    //let mut list = list;
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
            _ => panic!("TODO: ref does not have a type"),
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
                d.iter()
                    .map(|(key, val)| format!("{}: {}", key, val))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            RuValue::List(ls) => write!(
                f,
                "[{}]",
                ls.iter()
                    .map(|val| format!("{}", val))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            _ => unimplemented!(),
        }
    }
}

/*
impl<T> From<T> for RuValue
where
    T: Into<CoValue>,
{
    fn from(val: T) -> Self {
        instantiate(&val.into())
    }
}
*/

impl From<bool> for RuValue {
    fn from(b: bool) -> Self {
        RuValue::Bool(b)
    }
}

impl From<i64> for RuValue {
    fn from(n: i64) -> Self {
        RuValue::Int(n)
    }
}

impl From<f64> for RuValue {
    fn from(n: f64) -> Self {
        RuValue::Float(n)
    }
}

impl From<&str> for RuValue {
    fn from(s: &str) -> Self {
        RuValue::Str(s.to_string())
    }
}

impl<T> From<Vec<T>> for RuValue
where
    T: Into<RuValue>,
{
    fn from(val: Vec<T>) -> Self {
        RuValue::List(val.into_iter().map(T::into).collect())
    }
}

fn serialize_ruvalue_ref<S>(_: &Option<RuValueRef>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_none()
}

fn deserialize_ruvalue_ref<'de, D>(d: D) -> Result<Option<RuValueRef>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(None)
}
