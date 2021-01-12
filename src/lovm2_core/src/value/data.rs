//! Representation of values

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use super::*;

/// Reference to a generic lovm2 object
pub type AnyRef = Rc<RefCell<Handle>>;
/// Reference to a lovm2 [Value]
pub type ValueRef = Reference;

/// Wrap the given value inside a `Ref(_)`. `Dict` and `List` values will be wrapped deeply.
pub fn box_value(value: Value) -> Value {
    let outer = match value {
        Value::Dict(d) => {
            let mut hm = IndexMap::new();
            for (key, val) in d.into_iter() {
                if val.is_ref() {
                    hm.insert(key, val);
                } else {
                    hm.insert(key, box_value(val));
                }
            }
            Value::Dict(hm)
        }
        Value::List(l) => Value::List(
            l.into_iter()
                .map(|val| if val.is_ref() { val } else { box_value(val) })
                .collect::<Vec<_>>(),
        ),
        value => value,
    };
    Value::Ref(Reference::from(outer))
}

/// Runtime representation of values
#[derive(Clone, Deserialize, Serialize)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    #[serde(with = "indexmap::serde_seq")]
    Dict(IndexMap<Value, Value>),
    List(Vec<Value>),
    Ref(Reference),
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    Any(AnyRef),
}

impl Value {
    /// Create a `Handle` to the given value.
    pub fn create_any<T>(from: T) -> Self
    where
        T: std::any::Any,
    {
        Value::Any(Rc::new(RefCell::new(Handle(Box::new(from)))))
    }

    /// If the current value is an instance of `Ref`, this function
    /// will return an owned clone of the innermost value. If the value
    /// is not a reference, this is just a clone.
    pub fn clone_inner(&self) -> Lovm2Result<Value> {
        if let Value::Ref(r) = self {
            Ok(r.unref_to_value()?.borrow().clone())
        } else {
            Ok(self.clone())
        }
    }

    /// Ensure that the value is not wrapped in a reference.
    /// This is used for stack mutations as first operand.
    pub fn unref_inplace(&mut self) -> Lovm2Result<()> {
        if let Value::Ref(r) = self {
            *self = r.unref_to_value()?.borrow().clone();
        }
        Ok(())
    }

    /// Returns true if the value is a reference.
    pub fn is_ref(&self) -> bool {
        matches!(self, Value::Ref(_))
    }

    /// Create an iterator from the value. This will return
    /// an error if the value does not support iteration.
    pub fn iter(&self) -> Lovm2Result<iter::Iter> {
        iter::Iter::try_from(self.clone())
    }

    /// Returns a completely independent version of the value.
    /// This will recursively clone the items of `List` and `Dict`
    /// as well as `Ref`.
    pub fn deep_clone(&self) -> Self {
        match self {
            Value::Dict(d) => {
                let mut dc = Value::dict();
                for (key, val) in d.iter() {
                    dc.set(&key.clone(), val.clone()).unwrap();
                }
                box_value(dc)
            }
            Value::List(ls) => {
                let ls = ls.iter().map(Self::deep_clone).collect();
                box_value(Value::List(ls))
            }
            Value::Ref(r) => Value::Ref(r.deep_clone()),
            _ => self.clone(),
        }
    }

    /// Delete an item from a value by key.
    pub fn delete(&mut self, key: &Value) -> Lovm2Result<()> {
        match self {
            Value::Dict(dict) => {
                dict.remove(key);
            }
            Value::List(list) => {
                let key = key.as_integer_inner()?;
                list.remove(key as usize);
            }
            Value::Ref(r) => r.borrow_mut()?.delete(key)?,
            _ => return Err((Lovm2ErrorTy::OperationNotSupported, "delete").into()),
        }
        Ok(())
    }

    /// Retrieve an item by key.
    pub fn get(&self, key: &Value) -> Lovm2Result<Value> {
        match self {
            Value::Str(_) => self.get_by_index(key.as_integer_inner()? as usize),
            Value::Dict(dict) => match dict.get(key) {
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
            Value::Ref(r) => r.borrow()?.get(key),
            _ => Err((Lovm2ErrorTy::OperationNotSupported, "get").into()),
        }
    }

    /// Get an item by a number. This is mainly used for iteration.
    pub fn get_by_index(&self, idx: usize) -> Lovm2Result<Value> {
        match self {
            Value::Str(s) => s
                .chars()
                .nth(idx)
                .map(Value::from)
                .ok_or_else(|| (Lovm2ErrorTy::KeyNotFound, idx.to_string()).into()),
            Value::Dict(dict) => dict
                .get_index(idx)
                .map(|(key, val)| box_value(Value::List(vec![key.clone(), val.clone()])))
                .ok_or_else(|| (Lovm2ErrorTy::KeyNotFound, idx.to_string()).into()),
            Value::List(list) => list
                .get(idx)
                .cloned()
                .ok_or_else(|| (Lovm2ErrorTy::KeyNotFound, idx.to_string()).into()),
            Value::Ref(r) => r.borrow()?.get_by_index(idx),
            _ => Err((Lovm2ErrorTy::OperationNotSupported, "get_by_index").into()),
        }
    }

    /// Retrieve the length of the value.
    pub fn len(&self) -> Lovm2Result<usize> {
        match self {
            Value::Str(s) => Ok(s.len()),
            Value::Dict(dict) => Ok(dict.len()),
            Value::List(list) => Ok(list.len()),
            Value::Ref(r) => r.borrow()?.len(),
            _ => Err((Lovm2ErrorTy::OperationNotSupported, "len").into()),
        }
    }

    /// Insert a new item into the value.
    pub fn set(&mut self, key: &Value, mut val: Value) -> Lovm2Result<()> {
        if !val.is_ref() {
            val = box_value(val);
        }

        match self {
            Value::Dict(dict) => {
                dict.insert(key.clone(), val);
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
            Value::Ref(r) => r.borrow_mut()?.set(key, val),
            _ => Err((Lovm2ErrorTy::OperationNotSupported, "set").into()),
        }
    }
}

impl Value {
    /// Create a new instance of `Dict`.
    pub fn dict() -> Self {
        Self::Dict(IndexMap::new())
    }

    /// Create a new instance of `List`.
    pub fn list() -> Self {
        Self::List(vec![])
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
            Value::Ref(r) => {
                let r = r.borrow().unwrap();
                r.hash(hasher);
            }
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
            Value::Ref(r) => write!(f, "{}", r),
            Value::Any(_) => write!(f, "Handle"),
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Nil
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<char> for Value {
    fn from(c: char) -> Self {
        Value::Str(c.to_string())
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
            Value::Ref(r) => write!(f, "{:?}", r),
            Value::Any(_) => write!(f, "Handle"),
        }
    }
}

/// Value type mostly used to handle extern values
pub struct Handle(pub Box<dyn std::any::Any>);

impl PartialEq for Handle {
    fn eq(&self, other: &Self) -> bool {
        // TODO: try value compare
        self.0.type_id() == other.0.type_id()
    }
}
