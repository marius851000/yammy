use crate::errors::*;
use crate::Entry;
use crate::EntryValue;
use crate::TableData;

pub enum EntryBuilder<'a> {
    Ok(Entry, &'a TableData),
    Err(Error),
}

impl<'a> EntryBuilder<'a> {
    pub fn new(tabledata: &'a TableData) -> Self {
        EntryBuilder::Ok(Entry::new(tabledata), tabledata)
    }

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

    pub fn get(self) -> Result<Entry> {
        match self {
            EntryBuilder::Ok(entry, _) => Ok(entry),
            EntryBuilder::Err(err) => Err(err.chain_err(|| "building an Entry with EntryBuilder")),
        }
    }

    pub fn unwrap(self) -> Entry {
        self.get().unwrap()
    }
}

//TASK: add tests for EntryBuilder
