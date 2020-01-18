use super::Entry;
use super::Metadata;
use super::TableDataMap;
use super::ID;
use super::{ModRead, ModWrite};
use crate::errors::*;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::sync::Arc;

pub struct DefaultMod {
    metadata: Metadata,
    tabledatamap: Arc<TableDataMap>,
    modified_data: HashMap<String, HashMap<ID, Entry>>,
    removed_value: HashMap<String, BTreeSet<ID>>,
}

impl DefaultMod {
    pub fn new(metadata: Metadata, tabledatamap: Arc<TableDataMap>) -> DefaultMod {
        DefaultMod {
            metadata,
            tabledatamap,
            modified_data: HashMap::new(),
            removed_value: HashMap::new(),
        }
    }
}

impl ModRead for DefaultMod {
    fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn get_tabledatamap(&self) -> Arc<TableDataMap> {
        self.tabledatamap.clone()
    }

    fn is_writable(&self) -> bool {
        true
    }

    fn get_modified_table_list(&self) -> Vec<String> {
        let mut modified_table = Vec::new();
        for m in self.modified_data.keys() {
            modified_table.push(m.clone());
        }
        modified_table
    }

    fn get_modified_entry_list(&self, table: &str) -> Result<Vec<ID>> {
        if !self.modified_data.contains_key(table) {
            return Ok(Vec::new());
        };
        let mut entrys: Vec<ID> = Vec::new();
        for entry in self.modified_data[table].keys() {
            entrys.push(entry.clone());
        }
        Ok(entrys)
    }

    fn get_entry(&self, table: &str, id: &ID) -> Result<Option<&Entry>> {
        match self.modified_data.get(table) {
            None => Ok(None),
            Some(table_hashmap) => match table_hashmap.get(id) {
                None => Ok(None),
                Some(result) => Ok(Some(&result)),
            },
        }
    }

    fn is_removed(&self, table: &str, id: &ID) -> Result<bool> {
        if let Some(removed_table) = self.removed_value.get(table) {
            Ok(removed_table.contains(id))
        } else {
            Ok(false)
        }
    }

    fn list_removed(&self, table: &str) -> Result<BTreeSet<ID>> {
        if let Some(removed_table) = self.removed_value.get(table) {
            Ok(removed_table.clone())
        } else {
            Ok(BTreeSet::new())
        }
    }
}

impl ModWrite for DefaultMod {
    fn restore(&mut self, table: &str, id: &ID) -> Result<()> {
        if let Some(remove_table) = self.removed_value.get_mut(table) {
            remove_table.remove(id);
        };
        Ok(())
    }

    fn insert(&mut self, table: String, id: ID, value: Entry) -> Result<()> {
        let table_data = match self.tabledatamap.get(&table) {
                Some(value) => value,
                None => {
                    return Err(Error::from(
                        format!("can't create a new entryin the table {}: the table is not found in the table data map", table),
                    ))
            }
        };

        table_data.check(&value)?;

        // guaranted to add te value as of now
        self.restore(&table, &id)
            .chain_err(|| "error restoring an entry while trying to insert it")?;
        // create an entry in self.modified_data if it doesn't already exist
        if !self.modified_data.contains_key(&table) {
            self.modified_data.insert(table.clone(), HashMap::new());
        };
        self.modified_data
            .get_mut(&table)
            .unwrap()
            .insert(id, value);
        Ok(())
    }

    fn remove(&mut self, table: String, id: ID) -> Result<()> {
        if self.is_removed(&table, &id).unwrap() {
            return Ok(());
        };
        if let Some(modified_table) = self.modified_data.get_mut(&table) {
            if modified_table.contains_key(&id) {
                modified_table.remove(&id).unwrap();
            };
        };
        match self.removed_value.get_mut(&table) {
            Some(set_removed) => {
                set_removed.insert(id);
            }
            None => {
                let mut set_removed = BTreeSet::new();
                set_removed.insert(id);
                self.removed_value.insert(table, set_removed);
            }
        };
        Ok(())
    }
}

#[test]
fn test_defaultmod() {
    use super::EntryData;
    use super::EntryType;
    use super::EntryValue;

    use crate::builder::EntryBuilder;
    use crate::builder::TableDataBuilder;
    use crate::builder::TableDataMapBuilder;

    // const
    let partner_id = ID::String(String::from("partner"));
    // setup

    let tabledatamap = TableDataMapBuilder::new()
        .insert(
            String::from("chara"),
            TableDataBuilder::new()
                .add_data("name".into(), EntryData::new(EntryType::String))
                .add_data("pv".into(), EntryData::new(EntryType::Float64))
                .get(),
        )
        .insert(
            String::from("attack"),
            TableDataBuilder::new()
                .add_data("name".into(), EntryData::new(EntryType::String))
                .add_data("damage".into(), EntryData::new(EntryType::Unsigned64))
                .get(),
        )
        .get();

    let chara_table_data = &tabledatamap[String::from("chara")];
    let attack_table_data = &tabledatamap[String::from("attack")];

    // the test itself

    let mut metadata = Metadata::default();
    metadata.name = String::from("test_mod");

    let mut r#mod = DefaultMod::new(metadata, tabledatamap.clone());

    assert!(r#mod.is_writable());
    assert_eq!(r#mod.get_metadata().name, String::from("test_mod"));

    //test write
    let entry_soren = EntryBuilder::new(&chara_table_data)
        .set_key_by_string(
            String::from("name"),
            EntryValue::String(String::from("Soren")),
        )
        .unwrap();

    let entry_twilight = EntryBuilder::new(&chara_table_data)
        .set_key_by_string(
            String::from("name"),
            EntryValue::String(String::from("Twilight")),
        )
        .unwrap();

    assert_eq!(r#mod.get_modified_table_list().len(), 0);

    r#mod
        .insert(String::from("chara"), ID::Integer(1), entry_soren)
        .unwrap();
    r#mod
        .insert(
            String::from("chara"),
            partner_id.clone(),
            entry_twilight.clone(),
        )
        .unwrap();

    assert_eq!(r#mod.get_modified_table_list(), vec!["chara"]);

    let modified_entry = r#mod
        .get_modified_entry_list(&String::from("chara"))
        .unwrap();
    for should_contain in vec![ID::Integer(1), partner_id.clone()] {
        assert!(modified_entry.contains(&should_contain));
    }
    assert_eq!(modified_entry.len(), 2);
    assert!(r#mod
        .get_modified_entry_list(&String::from("unexisting"))
        .unwrap()
        .is_empty());

    // test an invalid insertion
    assert!(r#mod
        .insert(
            String::from("chara"),
            ID::String(String::from("battle claw")),
            EntryBuilder::new(&attack_table_data)
                .set_key_by_string(
                    String::from("name"),
                    EntryValue::String(String::from("battle claw")),
                )
                .set_key_by_string(String::from("damage"), EntryValue::Unsigned64(30))
                .unwrap()
        )
        .is_err());

    // test getting a value
    assert_eq!(
        r#mod
            .get_entry(&String::from("chara"), &partner_id)
            .unwrap()
            .unwrap()
            .get_key_by_string(&chara_table_data, String::from("name"))
            .unwrap()
            .get_string()
            .unwrap(),
        &String::from("Twilight")
    );

    // test removing a value
    r#mod
        .remove(String::from("chara"), partner_id.clone())
        .unwrap();
    assert!(r#mod
        .is_removed(&String::from("chara"), &partner_id)
        .unwrap());
    assert!(r#mod
        .list_removed(&String::from("chara"))
        .unwrap()
        .contains(&partner_id));
    assert!(r#mod
        .get_entry(&String::from("chara"), &partner_id)
        .unwrap()
        .is_none());
    // read
    r#mod
        .insert(
            String::from("chara"),
            partner_id.clone(),
            entry_twilight.clone(),
        )
        .unwrap();
    assert!(!r#mod
        .is_removed(&String::from("chara"), &partner_id)
        .unwrap());
    assert_eq!(r#mod.list_removed(&String::from("chara")).unwrap().len(), 0);
    assert!(r#mod
        .get_entry(&String::from("chara"), &partner_id)
        .unwrap()
        .is_some());
}
