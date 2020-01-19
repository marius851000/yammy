use crate::EntryData;
use crate::TableData;

#[must_use]
pub struct TableDataBuilder {
    table_data: TableData,
}

#[allow(clippy::new_without_default)]
/// A builder for [[TableData]]
impl TableDataBuilder {
    pub fn new() -> Self {
        TableDataBuilder {
            table_data: TableData::new(),
        }
    }

    /// Add a new [[EntryData]] in the [[TableData]]. Is equivalent to [[TableData::add_data]].
    pub fn add_data(mut self, str: String, entrydata: EntryData) -> Self {
        self.table_data.add_data(str, entrydata);
        self
    }

    /// Return the [[TableData]]
    pub fn get(self) -> TableData {
        self.table_data
    }
}

//TASK: add a test
