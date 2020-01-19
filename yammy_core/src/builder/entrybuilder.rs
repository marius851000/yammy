use crate::errors::*;
use crate::Entry;
use crate::EntryValue;
use crate::TableData;

/// A builder for an [Entry]. Error are reported at the end of the generation
#[must_use]
pub enum EntryBuilder<'a> {
    Ok(Entry, &'a TableData),
    Err(Error),
}

impl<'a> EntryBuilder<'a> {
    /// Create a new [EntryBuilder]
    ///
    /// The [TableData] input is a reference to the [TableData] this entry is placed into.
    pub fn new(tabledata: &'a TableData) -> Self {
        EntryBuilder::Ok(Entry::new(tabledata), tabledata)
    }

    /// define a key of the [Entry]. Is equivalent to [Entry::set_key], without having to pass a reference to [TableData].
    pub fn set_key(self, id: usize, value: EntryValue) -> Self {
        match self {
            EntryBuilder::Ok(mut entry, tabledata) => {
                if let Err(err) = entry.set_key(tabledata, id, value) {
                    EntryBuilder::Err(err)
                } else {
                    EntryBuilder::Ok(entry, tabledata)
                }
            }
            EntryBuilder::Err(err) => EntryBuilder::Err(err),
        }
    }

    /// define an entry by [String]. Is equivalent to [Entry::set_key_by_string], without having to pass a reference to [TableData].
    pub fn set_key_by_string(self, id: String, value: EntryValue) -> Self {
        match self {
            EntryBuilder::Ok(mut entry, tabledata) => {
                if let Err(err) = entry.set_key_by_string(tabledata, id, value) {
                    EntryBuilder::Err(err)
                } else {
                    EntryBuilder::Ok(entry, tabledata)
                }
            }
            EntryBuilder::Err(err) => EntryBuilder::Err(err),
        }
    }

    /// return the [Entry] if the process happened well, the error otherwise.
    pub fn get(self) -> Result<Entry> {
        match self {
            EntryBuilder::Ok(entry, _) => Ok(entry),
            EntryBuilder::Err(err) => Err(err.chain_err(|| "building an Entry with EntryBuilder")),
        }
    }

    /// return the [Entry]. Panic if there is an error. See [EntryBuilder::get] for a safe alternative.
    pub fn unwrap(self) -> Entry {
        self.get().unwrap()
    }
}

//TASK: add tests for EntryBuilder
