use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use super::*;

/// Reference to another value
#[derive(Clone, Deserialize, Serialize)]
pub struct LV2Reference(
    #[serde(serialize_with = "serialize_value_ref")]
    #[serde(deserialize_with = "deserialize_value_ref")]
    pub(crate) Option<Rc<RefCell<LV2Value>>>,
);

impl LV2Reference {
    /// Returns true if the reference is bound to a value.
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    /// Try to borrow the inner value.
    pub fn borrow(&self) -> LV2Result<Ref<'_, LV2Value>> {
        if let Some(inner) = &self.0 {
            Ok(inner.borrow())
        } else {
            err_from_string("value is already borrowed")
        }
    }

    /// Try to borrow the inner value as mutable.
    pub fn borrow_mut(&self) -> LV2Result<RefMut<'_, LV2Value>> {
        if let Some(inner) = &self.0 {
            Ok(inner.borrow_mut())
        } else {
            err_from_string("value is already borrowed")
        }
    }

    /// Dereference to the contained value. Nested references will
    /// be dereferenced until a non-reference value was found.
    pub fn unref_to_value(&self) -> LV2Result<Rc<RefCell<LV2Value>>> {
        if let Some(val) = &self.0 {
            if let LV2Value::Ref(r) = &*val.borrow() {
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

impl From<LV2Value> for LV2Reference {
    fn from(val: LV2Value) -> Self {
        Self(Some(Rc::new(RefCell::new(val))))
    }
}

fn serialize_value_ref<S>(_: &Option<Rc<RefCell<LV2Value>>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_none()
}

fn deserialize_value_ref<'de, D>(_: D) -> Result<Option<Rc<RefCell<LV2Value>>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(None)
}

impl std::cmp::PartialEq for LV2Reference {
    fn eq(&self, rhs: &Self) -> bool {
        self.unref_to_value().map_or(false, |lhs| {
            rhs.unref_to_value().map_or(false, |rhs| lhs == rhs)
        })
    }
}

impl std::cmp::PartialEq<LV2Value> for LV2Reference {
    fn eq(&self, rhs: &LV2Value) -> bool {
        if let LV2Value::Ref(rhs) = rhs {
            self == rhs
        } else {
            self.unref_to_value()
                .map_or(false, |lhs| *lhs.borrow() == *rhs)
        }
    }
}

impl std::fmt::Display for LV2Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(r) = &self.0 {
            write!(f, "{}", r.borrow())
        } else {
            write!(f, "None")
        }
    }
}

impl std::fmt::Debug for LV2Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(r) = &self.0 {
            write!(f, "Ref({:?})", r.borrow())
        } else {
            write!(f, "Ref(None)")
        }
    }
}
