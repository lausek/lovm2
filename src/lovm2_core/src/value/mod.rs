//! Representation and operations for [LV2Value].

mod conv;
mod op;
mod opi;
mod r#ref;

pub(crate) mod iter;

pub use self::conv::*;
pub use self::iter::LV2Iter;
pub use self::r#ref::LV2Reference;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use crate::error::*;

/// Reference to a generic lovm2 object.
pub type LV2AnyRef = Rc<RefCell<LV2Handle>>;
/// Reference to a lovm2 [LV2Value].
pub type LV2ValueRef = LV2Reference;

/// Value type mostly used to handle extern values.
pub struct LV2Handle(pub Box<dyn std::any::Any>);

impl PartialEq for LV2Handle {
    fn eq(&self, other: &Self) -> bool {
        // TODO: try value compare
        self.0.type_id() == other.0.type_id()
    }
}

/// Wrap the given value inside a [LV2Value::Ref]. [LV2Value::Dict] and [LV2Value::List] values will be wrapped deeply.
pub fn box_value(value: LV2Value) -> LV2Value {
    let outer = match value {
        LV2Value::Dict(d) => {
            let mut hm = IndexMap::new();

            for (key, val) in d.into_iter() {
                if val.is_ref() {
                    hm.insert(key, val);
                } else {
                    hm.insert(key, box_value(val));
                }
            }

            LV2Value::Dict(hm)
        }
        LV2Value::List(l) => LV2Value::List(
            l.into_iter()
                .map(|val| if val.is_ref() { val } else { box_value(val) })
                .collect::<Vec<_>>(),
        ),
        value => value,
    };

    LV2Value::Ref(LV2Reference::from(outer))
}

/// Runtime representation of values.
#[derive(Clone, Deserialize, Serialize)]
pub enum LV2Value {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),

    #[serde(with = "indexmap::serde_seq")]
    Dict(IndexMap<LV2Value, LV2Value>),
    List(Vec<LV2Value>),
    Ref(LV2Reference),

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    Iter(Rc<RefCell<LV2Iter>>),

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    Any(LV2AnyRef),
}

impl LV2Value {
    /// Append a new item to the value.
    pub fn append(&mut self, mut val: LV2Value) -> LV2Result<()> {
        if !val.is_ref() {
            val = box_value(val);
        }

        match self {
            LV2Value::List(list) => {
                list.push(val);
                Ok(())
            }
            LV2Value::Ref(r) => r.borrow_mut()?.append(val),
            _ => Err((LV2ErrorTy::OperationNotSupported, "append").into()),
        }
    }
    /// Create a [LV2Handle] to the given value.
    pub fn create_any<T>(from: T) -> Self
    where
        T: std::any::Any,
    {
        LV2Value::Any(Rc::new(RefCell::new(LV2Handle(Box::new(from)))))
    }

    /// If the current value is an instance of [LV2Value::Ref], this function
    /// will return an owned clone of the innermost value. If the value
    /// is not a reference, this is just a clone.
    pub fn clone_inner(&self) -> LV2Result<LV2Value> {
        if let LV2Value::Ref(r) = self {
            Ok(r.unref_to_value()?.borrow().clone())
        } else {
            Ok(self.clone())
        }
    }

    /// Ensure that the value is not wrapped in a reference.
    /// This is used for stack mutations as first operand.
    pub fn unref_inplace(&mut self) -> LV2Result<()> {
        if let LV2Value::Ref(r) = self {
            *self = r.unref_to_value()?.borrow().clone();
        }
        Ok(())
    }

    /// Returns true if the value is a reference.
    pub fn is_ref(&self) -> bool {
        matches!(self, LV2Value::Ref(_))
    }

    /// Create an iterator from the value. This will return
    /// an error if the value does not support iteration.
    pub fn iter(&self) -> LV2Result<iter::LV2Iter> {
        iter::LV2Iter::try_from(self.clone())
    }

    /// Returns a completely independent version of the value.
    /// This will recursively clone the items of [LV2Value::Dict] and [LV2Value::List]
    /// as well as [LV2Value::Ref].
    pub fn deep_clone(&self) -> Self {
        match self {
            LV2Value::Dict(d) => {
                let mut dc = LV2Value::dict();

                for (key, val) in d.iter() {
                    dc.set(&key.clone(), val.clone()).unwrap();
                }

                box_value(dc)
            }
            LV2Value::List(ls) => {
                let ls = ls.iter().map(Self::deep_clone).collect();

                box_value(LV2Value::List(ls))
            }
            LV2Value::Ref(r) => LV2Value::Ref(r.deep_clone()),
            _ => self.clone(),
        }
    }

    /// Delete an item from a value by key.
    pub fn delete(&mut self, key: &LV2Value) -> LV2Result<()> {
        match self {
            LV2Value::Dict(dict) => {
                dict.remove(key);
            }
            LV2Value::List(list) => {
                let key = key.as_integer_inner()?;

                list.remove(key as usize);
            }
            LV2Value::Ref(r) => r.borrow_mut()?.delete(key)?,
            _ => return Err((LV2ErrorTy::OperationNotSupported, "delete").into()),
        }
        Ok(())
    }

    /// Retrieve an item by key.
    pub fn get(&self, key: &LV2Value) -> LV2Result<LV2Value> {
        match self {
            LV2Value::Str(_) => self.get_by_index(key.as_integer_inner()? as usize),
            LV2Value::Dict(dict) => match dict.get(key) {
                Some(val) => Ok(val.clone()),
                None => err_key_not_found(self, &key)?,
            },
            LV2Value::List(list) => {
                if let LV2Value::Int(idx) = key.as_integer()? {
                    match list.get(idx as usize) {
                        Some(val) => Ok(val.clone()),
                        None => err_key_not_found(self, &key)?,
                    }
                } else {
                    unreachable!()
                }
            }
            LV2Value::Ref(r) => r.borrow()?.get(key),
            _ => Err((LV2ErrorTy::OperationNotSupported, "get").into()),
        }
    }

    /// Get an item by a number. This is mainly used for iteration.
    pub fn get_by_index(&self, idx: usize) -> LV2Result<LV2Value> {
        match self {
            LV2Value::Str(s) => s
                .chars()
                .nth(idx)
                .map(LV2Value::from)
                .ok_or_else(|| (LV2ErrorTy::KeyNotFound, idx.to_string()).into()),
            LV2Value::Dict(dict) => dict
                .get_index(idx)
                .map(|(key, val)| box_value(LV2Value::List(vec![key.clone(), val.clone()])))
                .ok_or_else(|| (LV2ErrorTy::KeyNotFound, idx.to_string()).into()),
            LV2Value::List(list) => list
                .get(idx)
                .cloned()
                .ok_or_else(|| (LV2ErrorTy::KeyNotFound, idx.to_string()).into()),
            LV2Value::Ref(r) => r.borrow()?.get_by_index(idx),
            _ => Err((LV2ErrorTy::OperationNotSupported, "get_by_index").into()),
        }
    }

    /// Retrieve the length of the value.
    pub fn len(&self) -> LV2Result<usize> {
        match self {
            LV2Value::Str(s) => Ok(s.len()),
            LV2Value::Dict(dict) => Ok(dict.len()),
            LV2Value::List(list) => Ok(list.len()),
            LV2Value::Ref(r) => r.borrow()?.len(),
            _ => Err((LV2ErrorTy::OperationNotSupported, "len").into()),
        }
    }

    /// Insert a new item into the value.
    pub fn set(&mut self, key: &LV2Value, mut val: LV2Value) -> LV2Result<()> {
        if !val.is_ref() {
            val = box_value(val);
        }

        match self {
            LV2Value::Dict(dict) => {
                dict.insert(key.clone(), val);
                Ok(())
            }
            LV2Value::List(list) => {
                let idx = key.as_integer_inner()?;

                if list.len() == idx as usize {
                    list.push(val);
                } else {
                    list[idx as usize] = val;
                }

                Ok(())
            }
            LV2Value::Ref(r) => r.borrow_mut()?.set(key, val),
            _ => Err((LV2ErrorTy::OperationNotSupported, "set").into()),
        }
    }
}

impl LV2Value {
    /// Create a new instance of [LV2Value::Dict].
    pub fn dict() -> Self {
        Self::Dict(IndexMap::new())
    }

    /// Create a new instance of [LV2Value::List].
    pub fn list() -> Self {
        Self::List(vec![])
    }
}

impl std::cmp::Eq for LV2Value {}

impl std::hash::Hash for LV2Value {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: std::hash::Hasher,
    {
        match self {
            LV2Value::Nil => unimplemented!(),
            LV2Value::Bool(b) => hasher.write_u8(*b as u8),
            LV2Value::Int(n) => hasher.write_i64(*n),
            LV2Value::Float(_) => unimplemented!(),
            LV2Value::Str(s) => hasher.write(s.as_bytes()),
            LV2Value::Dict(_) => unimplemented!(),
            LV2Value::List(_) => unimplemented!(),
            LV2Value::Ref(r) => {
                let r = r.borrow().unwrap();
                r.hash(hasher);
            }
            _ => panic!("TODO: ref does not have a type"),
        }
    }
}

impl std::fmt::Display for LV2Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LV2Value::Nil => write!(f, "Nil"),
            LV2Value::Bool(b) => write!(f, "{}", b),
            LV2Value::Int(n) => write!(f, "{}", n),
            LV2Value::Float(n) => write!(f, "{}", n),
            LV2Value::Str(s) => write!(f, "{}", s),
            LV2Value::Dict(d) => write!(
                f,
                "{{{}}}",
                d.iter()
                    .map(|(key, val)| format!("{}: {}", key, val))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            LV2Value::List(ls) => write!(
                f,
                "[{}]",
                ls.iter()
                    .map(|val| format!("{}", val))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            LV2Value::Ref(r) => write!(f, "{}", r),
            LV2Value::Iter(it) => write!(f, "{}", it.borrow()),
            LV2Value::Any(_) => write!(f, "Handle"),
        }
    }
}

impl From<()> for LV2Value {
    fn from(_: ()) -> Self {
        LV2Value::Nil
    }
}

impl From<bool> for LV2Value {
    fn from(b: bool) -> Self {
        LV2Value::Bool(b)
    }
}

impl From<char> for LV2Value {
    fn from(c: char) -> Self {
        LV2Value::Str(c.to_string())
    }
}

impl From<i64> for LV2Value {
    fn from(n: i64) -> Self {
        LV2Value::Int(n)
    }
}

impl From<f64> for LV2Value {
    fn from(n: f64) -> Self {
        LV2Value::Float(n)
    }
}

impl From<&str> for LV2Value {
    fn from(s: &str) -> Self {
        LV2Value::Str(s.to_string())
    }
}

impl From<String> for LV2Value {
    fn from(s: String) -> Self {
        LV2Value::Str(s)
    }
}

impl<T> From<Vec<T>> for LV2Value
where
    T: Into<LV2Value>,
{
    fn from(val: Vec<T>) -> Self {
        LV2Value::List(val.into_iter().map(T::into).collect())
    }
}

impl From<LV2Iter> for LV2Value {
    fn from(it: LV2Iter) -> Self {
        LV2Value::Iter(Rc::new(RefCell::new(it)))
    }
}

impl Into<bool> for LV2Value {
    fn into(self) -> bool {
        self.as_bool_inner().unwrap()
    }
}

impl Into<i64> for LV2Value {
    fn into(self) -> i64 {
        self.as_integer_inner().unwrap()
    }
}

impl Into<f64> for LV2Value {
    fn into(self) -> f64 {
        self.as_float_inner().unwrap()
    }
}

impl Into<String> for LV2Value {
    fn into(self) -> String {
        self.as_str_inner().unwrap()
    }
}

impl std::fmt::Debug for LV2Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            LV2Value::Nil => write!(f, "Nil"),
            LV2Value::Bool(b) => write!(f, "Bool({:?})", b),
            LV2Value::Int(n) => write!(f, "Int({:?})", n),
            LV2Value::Float(n) => write!(f, "Float({:?})", n),
            LV2Value::Str(s) => write!(f, "Str({:?})", s),
            LV2Value::Dict(m) => write!(f, "Dict({:?})", m),
            LV2Value::List(ls) => write!(f, "List({:?})", ls),
            LV2Value::Ref(r) => write!(f, "{:?}", r),
            LV2Value::Iter(it) => write!(f, "{:?}", it),
            LV2Value::Any(_) => write!(f, "Handle"),
        }
    }
}
