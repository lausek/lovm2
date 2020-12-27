use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use super::*;

#[derive(Clone, Deserialize, Serialize)]
pub struct Reference(
    #[serde(serialize_with = "serialize_value_ref")]
    #[serde(deserialize_with = "deserialize_value_ref")]
    Option<Rc<RefCell<Value>>>,
);

impl Reference {
    pub fn is_nil(&self) -> bool {
        self.0.is_none()
    }

    pub fn borrow(&self) -> Lovm2Result<Ref<'_, Value>> {
        if let Some(inner) = &self.0 {
            Ok(inner.borrow())
        } else {
            Err("".into())
        }
    }

    pub fn borrow_mut(&self) -> Lovm2Result<RefMut<'_, Value>> {
        if let Some(inner) = &self.0 {
            Ok(inner.borrow_mut())
        } else {
            Err("".into())
        }
    }

    pub fn unref(&self) -> Option<Value> {
        self.0.as_ref().map(|r| r.borrow().clone())
    }

    pub fn unref_total(&self) -> Lovm2Result<Value> {
        let mut val = self
            .unref()
            .ok_or_else(|| Lovm2Error::from("dereference on empty"))?;
        while let Value::Ref(r) = val {
            val = r
                .unref()
                .ok_or_else(|| Lovm2Error::from("dereference on empty"))?;
        }
        Ok(val)
    }
}

impl From<Value> for Reference {
    fn from(val: Value) -> Self {
        Self(Some(Rc::new(RefCell::new(val))))
    }
}

fn serialize_value_ref<S>(_: &Option<Rc<RefCell<Value>>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_none()
}

fn deserialize_value_ref<'de, D>(_: D) -> Result<Option<Rc<RefCell<Value>>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(None)
}

impl std::cmp::PartialEq for Reference {
    fn eq(&self, rhs: &Self) -> bool {
        self.unref_total().map_or(false, |lhs| {
            rhs.unref_total().map_or(false, |rhs| lhs == rhs)
        })
    }
}

impl std::cmp::PartialEq<Value> for Reference {
    fn eq(&self, rhs: &Value) -> bool {
        if let Value::Ref(rhs) = rhs {
            self == rhs
        } else {
            self.unref_total().map_or(false, |lhs| lhs == *rhs)
        }
    }
}

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(r) = &self.0 {
            write!(f, "{}", r.borrow())
        } else {
            write!(f, "None")
        }
    }
}

impl std::fmt::Debug for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(r) = &self.0 {
            write!(f, "Ref({:?})", r.borrow())
        } else {
            write!(f, "Ref(None)")
        }
    }
}
