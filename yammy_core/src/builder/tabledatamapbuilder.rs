use crate::TableData;
use crate::TableDataMap;
use std::sync::Arc;

pub struct TableDataMapBuilder {
    table_data_map: TableDataMap,
}

impl TableDataMapBuilder {
    pub fn new() -> Self {
        TableDataMapBuilder {
            table_data_map: TableDataMap::new(),
        }
    }

    pub fn insert(mut self, id: String, tabledata: TableData) -> Self {
        self.table_data_map.insert(id, tabledata);
        self
    }

    pub fn get(self) -> Arc<TableDataMap> {
        Arc::new(self.table_data_map)
    }
}

//TASK: add tests for TableDataMapBuilder
