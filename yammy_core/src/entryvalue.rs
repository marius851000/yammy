#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
/// Contain one variable of the following type:
/// String
/// Unsigned64 (u64)
/// Boolean (bool)
/// Float64 (f64)
pub enum EntryValue {
    String(String),
    Unsigned64(u64),
    Boolean(bool),
    Float64(f64),
}

impl EntryValue {
    /// Return Some(String) if this [EntryValue] contain a String, None otherwise
    pub fn get_string(&self) -> Option<&String> {
        match self {
            EntryValue::String(str) => Some(&str),
            _ => None,
        }
    }
    /// Return Some(u64) if this [EntryValue] contain an Unsigned64, None otherwise
    pub fn get_u64(&self) -> Option<u64> {
        match self {
            EntryValue::Unsigned64(number) => Some(*number),
            _ => None,
        }
    }
    /// Return Some(bool) if this [EntryValue] contain a Boolean, None otherwise
    pub fn get_bool(&self) -> Option<bool> {
        match self {
            EntryValue::Boolean(binary) => Some(*binary),
            _ => None,
        }
    }
    /// Return Some(f64) if this [EntryValue] contain a Float64, None otherwise
    pub fn get_f64(&self) -> Option<f64> {
        match self {
            EntryValue::Float64(number) => Some(*number),
            _ => None,
        }
    }
}

#[test]
fn test_entryvalue_get() {
    let s = EntryValue::String(String::from("Hello, World"));
    assert_eq!(s.get_string().unwrap(), &String::from("Hello, World"));

    let u64 = EntryValue::Unsigned64(42);
    assert_eq!(u64.get_u64().unwrap(), 42);

    for tested_boolean in [true, false].iter() {
        let b = EntryValue::Boolean(*tested_boolean);
        assert_eq!(b.get_bool().unwrap(), *tested_boolean);
    }

    let f64 = EntryValue::Float64(3.14);
    assert_eq!(f64.get_f64().unwrap(), 3.14);
}
