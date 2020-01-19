use super::EntryValue;
use crate::errors::*;

#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
/// The various type an [`EntryValue`] can hold
pub enum EntryType {
    /// A [`String`]
    String,
    /// An [`u64`]
    Unsigned64,
    /// A [`f64`]
    Float64,
    /// A [`bool`]
    Boolean,
}

#[derive(Debug, PartialEq, Clone)]
/// Information about a column in a [`crate::TableData`]
pub struct EntryData {
    entrytype: EntryType,
    default: Option<EntryValue>,
}

impl EntryData {
    /// Create a new [`EntryData`], of type [`EntryType`]
    pub fn new(entrytype: EntryType) -> EntryData {
        EntryData {
            entrytype,
            default: None,
        }
    }

    /// Set the default value of this [`EntryData`]
    pub fn default(mut self, default: EntryValue) -> Result<EntryData> {
        self.check(&default)?;
        self.default = Some(default);
        Ok(self)
    }

    /// Return the default value of this [`EntryData`].
    ///
    /// If not set, return a sensible default value according with its type:
    /// - An empty string for [`EntryType::String`]
    /// - 0 for numerical value
    /// - false for [`EntryType::Boolean`]
    pub fn get_default(&self) -> EntryValue {
        match &self.default {
            Some(default) => default.clone(),
            None => match self.entrytype {
                EntryType::String => EntryValue::String(String::new()),
                EntryType::Unsigned64 => EntryValue::Unsigned64(0),
                EntryType::Float64 => EntryValue::Float64(0.0),
                EntryType::Boolean => EntryValue::Boolean(false),
            },
        }
    }

    /// Return [`Ok`] if the [`EntryValue`] correspond with this [`EntryData`], [`Err`] with the reason otherwise
    pub fn check(&self, value: &EntryValue) -> Result<()> {
        match self.entrytype {
            EntryType::String => match value {
                EntryValue::String(_) => Ok(()),
                _ => Err(Error::from("This is not a string")),
            },
            EntryType::Unsigned64 => match value {
                EntryValue::Unsigned64(_) => Ok(()),
                _ => Err(Error::from("This is not an Unsigned64")),
            },
            EntryType::Boolean => match value {
                EntryValue::Boolean(_) => Ok(()),
                _ => Err(Error::from("this is not a boolean")),
            },
            EntryType::Float64 => match value {
                EntryValue::Float64(_) => Ok(()),
                _ => Err(Error::from("this is not a float64")),
            },
        }
    }
}

#[test]
fn test_entry_data_check() {
    assert!(EntryData::new(EntryType::String)
        .check(&EntryValue::String(String::from("hello")))
        .is_ok());
    assert!(EntryData::new(EntryType::Unsigned64)
        .check(&EntryValue::Unsigned64(42))
        .is_ok());
    assert!(EntryData::new(EntryType::Float64)
        .check(&EntryValue::Float64(3.14))
        .is_ok());
    assert!(EntryData::new(EntryType::Boolean)
        .check(&EntryValue::Boolean(true))
        .is_ok());
    assert!(EntryData::new(EntryType::String)
        .check(&EntryValue::Float64(3.14))
        .is_err());
}

#[test]
fn test_entry_data_default() {
    assert_eq!(
        EntryData::new(EntryType::Unsigned64)
            .get_default()
            .get_u64()
            .unwrap(),
        0
    );
    assert_eq!(
        EntryData::new(EntryType::Float64)
            .default(EntryValue::Float64(3.14))
            .unwrap()
            .get_default()
            .get_f64()
            .unwrap(),
        3.14
    );
}
