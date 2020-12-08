//! representation of values

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use lovm2_error::*;

pub type ValueRef = Rc<RefCell<Value>>;

/// wrap the given value inside a `Ref(_)`. `Dict` and `List` values will be wrapped deeply.
pub fn box_value(value: Value) -> Value {
    let outer = match value {
        Value::Dict(d) => {
            let mut hm = HashMap::new();
            for (key, val) in d.into_iter() {
                if let Value::Ref(_) = val {
                    hm.insert(key, val);
                } else {
                    hm.insert(key, box_value(val));
                }
            }
            Value::Dict(hm)
        }
        Value::List(l) => Value::List(
            l.into_iter()
                .map(|val| {
                    if let Value::Ref(_) = val {
                        val
                    } else {
                        box_value(val)
                    }
                })
                .collect::<Vec<_>>(),
        ),
        value => value,
    };
    Value::Ref(Some(Rc::new(RefCell::new(outer))))
}

/// runtime representation of values
#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Dict(HashMap<Value, Value>),
    List(Vec<Value>),
    #[serde(serialize_with = "serialize_ruvalue_ref")]
    #[serde(deserialize_with = "deserialize_ruvalue_ref")]
    Ref(Option<ValueRef>),
}

impl Value {
    pub fn deref(&self) -> Option<Value> {
        match self {
            Value::Ref(Some(r)) => Some(r.borrow().clone()),
            _ => None,
        }
    }

    pub fn is_ref(&self) -> bool {
        matches!(self, Value::Ref(_))
    }

    pub fn delete(&mut self, key: Value) -> Lovm2Result<()> {
        match self {
            Value::Dict(dict) => {
                dict.remove(&key);
            }
            Value::List(list) => {
                let key = key.as_integer_inner()?;
                list.remove(key as usize);
            }
            Value::Ref(Some(r)) => r.borrow_mut().delete(key)?,
            _ => return Err((Lovm2ErrorTy::OperationNotSupported, "delete").into()),
        }
        Ok(())
    }

    pub fn get(&self, key: Value) -> Lovm2Result<Value> {
        match self {
            Value::Dict(dict) => match dict.get(&key) {
                Some(val) => Ok(val.clone()),
                None => Err((Lovm2ErrorTy::KeyNotFound, key.to_string()).into()),
            },
            Value::List(list) => {
                if let Value::Int(key) = key.as_integer()? {
                    match list.get(key as usize) {
                        Some(val) => Ok(val.clone()),
                        None => Err((Lovm2ErrorTy::KeyNotFound, key.to_string()).into()),
                    }
                } else {
                    unreachable!()
                }
            }
            Value::Ref(Some(r)) => r.borrow().get(key),
            _ => Err((Lovm2ErrorTy::OperationNotSupported, "get").into()),
        }
    }

    pub fn len(&self) -> Lovm2Result<usize> {
        match self {
            Value::Dict(dict) => Ok(dict.len()),
            Value::List(list) => Ok(list.len()),
            Value::Ref(Some(r)) => r.borrow().len(),
            _ => Err((Lovm2ErrorTy::OperationNotSupported, "len").into()),
        }
    }

    pub fn set(&mut self, key: Value, mut val: Value) -> Lovm2Result<()> {
        if !val.is_ref() {
            val = box_value(val);
        }

        match self {
            Value::Dict(dict) => {
                dict.insert(key, val);
                Ok(())
            }
            Value::List(list) => {
                let idx = key.as_integer_inner()?;
                if list.len() == idx as usize {
                    list.push(val);
                } else {
                    list[idx as usize] = val;
                }
                Ok(())
            }
            Value::Ref(Some(r)) => r.borrow_mut().set(key, val),
            _ => Err((Lovm2ErrorTy::OperationNotSupported, "set").into()),
        }
    }
}

impl std::cmp::Eq for Value {}

impl std::hash::Hash for Value {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: std::hash::Hasher,
    {
        match self {
            Value::Nil => unimplemented!(),
            Value::Bool(b) => hasher.write_u8(*b as u8),
            Value::Int(n) => hasher.write_i64(*n),
            Value::Float(_) => unimplemented!(),
            Value::Str(s) => hasher.write(s.as_bytes()),
            Value::Dict(_) => unimplemented!(),
            Value::List(_) => unimplemented!(),
            _ => panic!("TODO: ref does not have a type"),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, "{}", s),
            Value::Dict(d) => write!(
                f,
                "{{{}}}",
                d.iter()
                    .map(|(key, val)| format!("{}: {}", key, val))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::List(ls) => write!(
                f,
                "[{}]",
                ls.iter()
                    .map(|val| format!("{}", val))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::Ref(Some(r)) => write!(f, "Ref({})", r.borrow()),
            Value::Ref(None) => write!(f, "Ref(None)"),
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Int(n)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Float(n)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Str(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Str(s)
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(val: Vec<T>) -> Self {
        Value::List(val.into_iter().map(T::into).collect())
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        self.as_bool_inner().unwrap()
    }
}

impl Into<i64> for Value {
    fn into(self) -> i64 {
        self.as_integer_inner().unwrap()
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        self.as_float_inner().unwrap()
    }
}

impl Into<String> for Value {
    fn into(self) -> String {
        self.as_str_inner().unwrap()
    }
}

fn serialize_ruvalue_ref<S>(_: &Option<ValueRef>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_none()
}

fn deserialize_ruvalue_ref<'de, D>(_: D) -> Result<Option<ValueRef>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(None)
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Bool(b) => write!(f, "Bool({:?})", b),
            Value::Int(n) => write!(f, "Int({:?})", n),
            Value::Float(n) => write!(f, "Float({:?})", n),
            Value::Str(s) => write!(f, "Str({:?})", s),
            Value::Dict(m) => write!(f, "Dict({:?})", m),
            Value::List(ls) => write!(f, "List({:?})", ls),
            Value::Ref(Some(r)) => write!(f, "Ref({:?})", r.borrow()),
            Value::Ref(None) => write!(f, "Ref(None)"),
        }
    }
}
