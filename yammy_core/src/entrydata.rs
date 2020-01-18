use super::EntryValue;
use crate::errors::*;

#[non_exhaustive]
#[derive(Debug)]
pub enum EntryType {
    String,
    Unsigned64,
    Float64,
    Boolean,
}

#[derive(Debug)]
pub struct EntryData {
    entrytype: EntryType,
    reference: Option<String>,
    default: Option<EntryValue>,
}

impl EntryData {
    pub fn new(entrytype: EntryType) -> EntryData {
        EntryData {
            entrytype,
            reference: None,
            default: None,
        }
    }

    pub fn reference(mut self, reference: String) -> EntryData {
        self.reference = Some(reference);
        self
    }

    pub fn default(mut self, default: EntryValue) -> Result<EntryData> {
        self.check(&default)?;
        self.default = Some(default);
        Ok(self)
    }

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
