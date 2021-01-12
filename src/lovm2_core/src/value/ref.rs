use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use super::*;

/// Reference to another value
#[derive(Clone, Deserialize, Serialize)]
pub struct Reference(
    #[serde(serialize_with = "serialize_value_ref")]
    #[serde(deserialize_with = "deserialize_value_ref")]
    pub(crate) Option<Rc<RefCell<Value>>>,
);

impl Reference {
    /// Returns true if the reference is bound to a value.
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    /// Try to borrow the inner value.
    pub fn borrow(&self) -> Lovm2Result<Ref<'_, Value>> {
        if let Some(inner) = &self.0 {
            Ok(inner.borrow())
        } else {
            err_from_string("value is already borrowed")
        }
    }

    /// Try to borrow the inner value as mutable.
    pub fn borrow_mut(&self) -> Lovm2Result<RefMut<'_, Value>> {
        if let Some(inner) = &self.0 {
            Ok(inner.borrow_mut())
        } else {
            err_from_string("value is already borrowed")
        }
    }

    /// Dereference to the contained value. Nested references will
    /// be dereferenced until a non-reference value was found.
    pub fn unref_to_value(&self) -> Lovm2Result<Rc<RefCell<Value>>> {
        if let Some(val) = &self.0 {
            if let Value::Ref(r) = &*val.borrow() {
                r.unref_to_value()
            } else {
                Ok(val.clone())
            }
        } else {
            err_empty_dereference()
        }
    }

    /// Replicate into an independet reference.
    pub fn deep_clone(&self) -> Self {
        if let Some(val) = &self.0 {
            Self::from(val.borrow().deep_clone())
        } else {
            Self(None)
        }
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
        self.unref_to_value().map_or(false, |lhs| {
            rhs.unref_to_value().map_or(false, |rhs| lhs == rhs)
        })
    }
}

impl std::cmp::PartialEq<Value> for Reference {
    fn eq(&self, rhs: &Value) -> bool {
        if let Value::Ref(rhs) = rhs {
            self == rhs
        } else {
            self.unref_to_value()
                .map_or(false, |lhs| *lhs.borrow() == *rhs)
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
