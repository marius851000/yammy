use crate::errors::*;
use crate::DefaultMod;
use crate::Entry;
use crate::Metadata;
use crate::ModWrite;
use crate::TableDataMap;
use crate::ID;
use std::sync::Arc;

/// A builder for [DefaultMod]. Mainly avalaible for testing purpose, but can be used for other goal.
pub enum DefaultModBuilder {
    Mod(DefaultMod),
    Broken(Error),
}

impl DefaultModBuilder {
    pub fn new(metadata: Metadata, tabledatamap: Arc<TableDataMap>) -> DefaultModBuilder {
        DefaultModBuilder::Mod(DefaultMod::new(metadata, tabledatamap))
    }

    pub fn insert(self, table: String, id: ID, value: Entry) -> DefaultModBuilder {
        match self {
            Self::Mod(mut actual_mod) => {
                if let Err(err) = actual_mod.insert(table, id, value) {
                    DefaultModBuilder::Broken(err)
                } else {
                    DefaultModBuilder::Mod(actual_mod)
                }
            }
            Self::Broken(err) => DefaultModBuilder::Broken(err),
        }
    }

    pub fn remove(self, table: String, id: ID) -> DefaultModBuilder {
        match self {
            Self::Mod(mut actual_mod) => {
                if let Err(err) = actual_mod.remove(table, id) {
                    DefaultModBuilder::Broken(err)
                } else {
                    DefaultModBuilder::Mod(actual_mod)
                }
            }
            Self::Broken(err) => DefaultModBuilder::Broken(err),
        }
    }

    pub fn get(self) -> Result<DefaultMod> {
        match self {
            Self::Mod(actual_mod) => Ok(actual_mod),
            Self::Broken(err) => Err(err.chain_err(|| "constructing a DefaultMod from a builder")),
        }
    }

    pub fn unwrap(self) -> DefaultMod {
        self.get().unwrap()
    }
}

#[test]
fn test_default_mod_builder() {
    use super::EntryBuilder;
    use super::TableDataBuilder;
    use super::TableDataMapBuilder;
    use crate::EntryData;
    use crate::EntryType;
    use crate::EntryValue;
    use crate::ModRead;
    let tabledatamap = TableDataMapBuilder::new()
        .insert(
            "chara".into(),
            TableDataBuilder::new()
                .add_data("name".into(), EntryData::new(EntryType::String))
                .add_data("pv".into(), EntryData::new(EntryType::Unsigned64))
                .get(),
        )
        .insert(
            "attack".into(),
            TableDataBuilder::new()
                .add_data("name".into(), EntryData::new(EntryType::String))
                .add_data("dmg".into(), EntryData::new(EntryType::Unsigned64))
                .get(),
        )
        .get();

    let chara_tabledata = &tabledatamap[String::from("chara")];
    let chara_name_id = chara_tabledata.string_to_id(String::from("name")).unwrap();
    let chara_pv_id = chara_tabledata.string_to_id(String::from("pv")).unwrap();
    //###########//

    let r#mod = DefaultModBuilder::new(Metadata::default(), tabledatamap.clone())
        .insert(
            "chara".into(),
            ID::String("hero".into()),
            EntryBuilder::new(&chara_tabledata)
                .set_key(chara_name_id, EntryValue::String(String::from("Soren")))
                .set_key(chara_pv_id, EntryValue::Unsigned64(300))
                .unwrap(),
        )
        .insert(
            "chara".into(),
            ID::Integer(100),
            EntryBuilder::new(&chara_tabledata)
                .set_key(chara_name_id, EntryValue::String(String::from("Twilight")))
                .set_key(chara_pv_id, EntryValue::Unsigned64(32))
                .unwrap(),
        )
        .remove("chara".into(), ID::String("Ezylryb".into()))
        .unwrap();

    let modified_chara_list = r#mod.get_modified_entry_list("chara".into()).unwrap();
    assert!(modified_chara_list.contains(&ID::Integer(100)));
    assert!(modified_chara_list.contains(&ID::String("hero".into())));
    assert!(r#mod
        .list_removed("chara".into())
        .unwrap()
        .contains(&ID::String("Ezylryb".into())));

    assert!(
        DefaultModBuilder::new(Metadata::default(), tabledatamap.clone())
            .insert(
                "unexistant".into(),
                ID::String("something".into()),
                EntryBuilder::new(&chara_tabledata).unwrap()
            )
            .get()
            .is_err()
    );
}
