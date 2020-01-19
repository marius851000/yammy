use super::EntryValue;
use super::TableData;
use crate::errors::*;

/// An entry of a [`super::Mod`]. It correspond to an entry in Table, with definition provided by a [`TableData`].
///
/// For optimisation reason, it doesn't store the [`TableData`], that should be provided when needed.
/// Internally, it store it's value in a [`Vec`]. The [`TableData`] hold the [`String`] key.
#[derive(Debug, PartialEq, Clone)]
pub struct Entry {
    values: Vec<EntryValue>,
}

#[allow(clippy::len_without_is_empty)]
impl Entry {
    ///Create a new [`Entry`], with for the provided [`TableData`], and use the default value from it.
    pub fn new(table_data: &TableData) -> Self {
        let mut values = Vec::new();
        for value_id in 0..table_data.len() {
            let entrydata = table_data.get_entrydata(value_id).unwrap(); //Assume to suceed
            values.push(entrydata.get_default());
        }
        Entry { values }
    }
    ///Get a value by its internal id
    pub fn get_key(&self, id: usize) -> Result<EntryValue> {
        if id < self.values.len() {
            Ok(self.values[id].clone())
        } else {
            Err(Error::from(
                "The id numeric is out of bound, and so it is impossible to get the value",
            ))
        }
    }
    ///Get a value by its string id ([`Entry::get_key`] is faster, but less practical in some case)
    pub fn get_key_by_string(&self, tabledata: &TableData, str: String) -> Result<EntryValue> {
        match tabledata.string_to_id(str) {
            Some(id) => self.get_key(id), //Assume to success
            None => Err(Error::from("The string key doesn't exist")),
        }
    }
    ///Set a value by its numeric id
    pub fn set_key(&mut self, tabledata: &TableData, id: usize, value: EntryValue) -> Result<()> {
        if id < self.values.len() {
            //Check if the value is correct:
            tabledata.get_entrydata(id).unwrap().check(&value)?;
            self.values[id] = value;
            Ok(())
        } else {
            Err(Error::from("The given numeric id is not correct"))
        }
    }
    /// Set a value by its string id (see also [`Entry::set_key`])
    pub fn set_key_by_string(
        &mut self,
        tabledata: &TableData,
        str: String,
        value: EntryValue,
    ) -> Result<()> {
        match tabledata.string_to_id(str) {
            Some(id) => self.set_key(tabledata, id, value),
            None => Err(Error::from("The string key doesn't exist")),
        }
    }

    /// Return the number of element this [`Entry`] hold. Should be the same to the one provided by [`TableData::len`]
    pub fn len(&self) -> usize {
        self.values.len()
    }
}

#[test]
fn test_entry() {
    use super::EntryData;
    use super::EntryType;
    let mut tabledata = TableData::new();
    tabledata.add_data(String::from("name"), EntryData::new(EntryType::String));
    tabledata.add_data(
        String::from("pv"),
        EntryData::new(EntryType::Float64)
            .default(EntryValue::Float64(10.0))
            .unwrap(),
    );

    let name_id = tabledata.string_to_id(String::from("name")).unwrap();
    let pv_id = tabledata.string_to_id(String::from("pv")).unwrap();

    let mut entry = Entry::new(&tabledata);
    assert_eq!(
        entry.get_key(name_id).unwrap().get_string().unwrap(),
        &String::from("")
    );
    entry
        .set_key(
            &tabledata,
            name_id,
            EntryValue::String(String::from("Soren")),
        )
        .unwrap();
    assert_eq!(
        entry
            .get_key_by_string(&tabledata, String::from("name"))
            .unwrap()
            .get_string()
            .unwrap(),
        &String::from("Soren")
    );

    entry
        .set_key_by_string(&tabledata, String::from("pv"), EntryValue::Float64(3.14))
        .unwrap();
    assert_eq!(
        entry
            .get_key_by_string(&tabledata, String::from("pv"))
            .unwrap()
            .get_f64()
            .unwrap(),
        3.14
    );

    entry
        .set_key(&tabledata, pv_id, EntryValue::Float64(45.0))
        .unwrap();
    assert!(entry
        .set_key(&tabledata, pv_id, EntryValue::Boolean(false))
        .is_err());
}
