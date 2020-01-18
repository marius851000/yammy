use crate::EntryData;
use crate::TableData;

pub struct TableDataBuilder {
    table_data: TableData,
}

#[allow(clippy::new_without_default)]
impl TableDataBuilder {
    pub fn new() -> Self {
        TableDataBuilder {
            table_data: TableData::new(),
        }
    }

    pub fn add_data(mut self, str: String, entrydata: EntryData) -> Self {
        self.table_data.add_data(str, entrydata);
        self
    }

    pub fn get(self) -> TableData {
        self.table_data
    }
}

//TASK: add a test
