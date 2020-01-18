use super::TableData;
use std::collections::HashMap;
use std::ops::Index;

#[derive(Default)]
pub struct TableDataMap {
    map: HashMap<String, TableData>,
}

impl TableDataMap {
    pub fn new() -> TableDataMap {
        Self::default()
    }

    pub fn insert(&mut self, id: String, tabledata: TableData) {
        self.map.insert(id, tabledata);
    }

    pub fn contains_key(&self, id: &str) -> bool {
        self.map.contains_key(id)
    }

    pub fn get(&self, id: &str) -> Option<&TableData> {
        self.map.get(id)
    }
}

impl Index<String> for TableDataMap {
    type Output = TableData;
    fn index(&self, key: String) -> &Self::Output {
        &self.map[&key]
    }
}

#[test]
fn test_tabledatamap() {
    use super::{EntryData, EntryType};
    let mut tdm = TableDataMap::new();
    let td0 = TableData::new();
    let mut td1 = TableData::new();
    td1.add_data(String::from("e1"), EntryData::new(EntryType::Unsigned64));
    tdm.insert(String::from("0"), td0);
    tdm.insert(String::from("1"), td1);
    assert_eq!(tdm[String::from("0")].len(), 0);
    assert_eq!(tdm[String::from("1")].len(), 1);
}
