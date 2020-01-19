use crate::TableData;
use crate::TableDataMap;
use std::sync::Arc;

#[must_use]
pub struct TableDataMapBuilder {
    table_data_map: TableDataMap,
}

#[allow(clippy::new_without_default)]
/// A builder for [TableDataMap]
impl TableDataMapBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        TableDataMapBuilder {
            table_data_map: TableDataMap::new(),
        }
    }

    /// Insert a new [TableData] in the [TableDataMap]. Is equivalent to [TableDataMap::insert]
    pub fn insert(mut self, id: String, tabledata: TableData) -> Self {
        self.table_data_map.insert(id, tabledata);
        self
    }

    /// Return the [TableDataMap], in an [Arc]
    pub fn get(self) -> Arc<TableDataMap> {
        Arc::new(self.table_data_map)
    }
}

//TASK: add tests for TableDataMapBuilder
