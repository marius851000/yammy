use super::Entry;
use super::EntryData;
use crate::errors::*;

//TODO: definie a table
/// Store the data about a table in the mod.
///
/// It have a builder [crate::builder::TableDataBuilder]
///
/// Internally, the various [EntryData] are stored in a Vec, and so have an numerical id ([usize]) associated to them.
/// It allow to optimize memory comsuption, as [Entry] just need to store their data in a Vec<Entry>, rather than a HashMap<String, Entry>
///
/// Some function still allow to pass a string to specify a column of the table, for convenience reason
///
/// Note: a data entry cannot be deleted, as it allow to do some assertion allowind to save memory
///
/// # Examples
///
/// ```
/// use yammy_core::{TableData, EntryData, EntryType};
/// //NOTE: it is better to use the builder for this
/// let mut table_data = TableData::new();
/// table_data.add_data("name".into(), EntryData::new(EntryType::String));
/// table_data.add_data("pv".into(), EntryData::new(EntryType::String));
/// let table_data = table_data;
/// let name_id = table_data.string_to_id("name".into()).unwrap();
/// assert_eq!(table_data.id_to_string(name_id).unwrap(), String::from("name"));
/// assert_eq!(table_data.get_entrydata(name_id).unwrap(), &EntryData::new(EntryType::String));
/// ```
#[derive(Default)]
pub struct TableData {
    id_counter: usize,
    strings: Vec<String>,
    entrydatas: Vec<EntryData>,
}

#[allow(clippy::len_without_is_empty)]
impl TableData {
    /// Create a new [TableData]
    pub fn new() -> TableData {
        Self::default()
    }

    /// Indicate the number of entry there is in this [TableData]
    pub fn len(&self) -> usize {
        self.id_counter
    }

    /// Add a data entry in this [TableData].
    ///
    /// It is added to the end of the column.
    pub fn add_data(&mut self, str: String, entrydata: EntryData) {
        let id = self.id_counter;
        debug_assert_eq!(self.strings.len(), id);
        debug_assert_eq!(self.entrydatas.len(), id);
        self.id_counter += 1;
        self.strings.push(str);
        self.entrydatas.push(entrydata);
    }

    /// Return the id corresponding to the given String if it exist
    pub fn string_to_id(&self, str: String) -> Option<usize> {
        for id in 0..self.strings.len() {
            if self.strings[id] == str {
                return Some(id);
            };
        }
        None
    }

    /// Return the String corresponding to the given id, if it exist
    pub fn id_to_string(&self, id: usize) -> Option<String> {
        if id < self.id_counter {
            Some(self.strings[id].clone())
        } else {
            None
        }
    }

    /// Return the [EntryData] associated with the id in this [TableData]
    pub fn get_entrydata(&self, id: usize) -> Option<&EntryData> {
        if id < self.id_counter {
            Some(&self.entrydatas[id])
        } else {
            None
        }
    }

    /// Check if the entry is valid for this [TableData]. Return an error if it doesn't.
    pub fn check(&self, entry: &Entry) -> Result<()> {
        if entry.len() != self.len() {
            return Err(Error::from(
                "the number of element of the tested entry and the table data does not correspond",
            ));
        };
        for key in 0..entry.len() {
            let value = entry
                .get_key(key)
                .chain_err(|| "impossible to get a key to check it")?;
            let entry_data = self
                .get_entrydata(key)
                .chain_err(|| "impossible to get an entrydata while checking an entry")?;
            entry_data
                .check(&value)
                .chain_err(|| "invalid value while found while checking an entry")?;
        }
        Ok(())
    }
}

#[test]
fn test_tabledata() {
    use super::EntryType;
    let mut tabledata = TableData::new();
    assert_eq!(tabledata.len(), 0);
    assert!(tabledata.id_to_string(0).is_none());
    assert!(tabledata.string_to_id(String::from("hello")).is_none());
    tabledata.add_data(String::from("test"), EntryData::new(EntryType::Unsigned64));
    assert_eq!(tabledata.len(), 1);
    assert_eq!(tabledata.id_to_string(0).unwrap(), String::from("test"));
    assert_eq!(tabledata.string_to_id(String::from("test")).unwrap(), 0);
    tabledata.add_data(String::from("hello"), EntryData::new(EntryType::Float64));
    assert_eq!(tabledata.string_to_id(String::from("hello")).unwrap(), 1);
    assert_eq!(tabledata.len(), 2);
    let default_entry = Entry::new(&tabledata);
    tabledata.check(&default_entry).unwrap()
}
