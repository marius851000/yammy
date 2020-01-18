use super::Entry;
use super::Game;
use super::ID;
use super::{ModRead, ModWrite};
use crate::errors::*;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;

/// A collection of [Mod]. The [Entry] it return when queried are the one of the most important [Mod].
///
/// Order of importance :
/// 1. The current mod (the one which is Read/Write)
/// 2. The static mod, the first being the most important
/// 3. The [Game]'s mod
pub struct ModPack {
    game: Arc<dyn Game>,
    static_mods: VecDeque<Arc<dyn ModRead>>,
    /// The [Mod] that is currently modified
    current_mod: Arc<Mutex<dyn ModWrite>>,
}

impl ModPack {
    pub fn new(game: Arc<dyn Game>, current_mod: Arc<Mutex<dyn ModWrite>>) -> ModPack {
        ModPack {
            game,
            static_mods: VecDeque::new(),
            current_mod,
        }
    }

    /// Add a [Mod] to static_mods
    ///
    /// The mod will be the first element of the static mod, thus being the second most important mod (behind the current mod)
    pub fn insert_mod(&mut self, r#mod: Arc<dyn ModRead>) {
        self.static_mods.push_front(r#mod);
    }

    pub fn get_entry(&self, table: &str, id: &ID) -> Result<Option<Entry>> {
        let current_mod = match self.current_mod.lock() {
            Ok(v) => v,
            Err(_) => return Err(Error::from("Impossible to lock the current mod")),
        };
        if current_mod
            .is_removed(table, id)
            .chain_err(|| "Impossible to check if an element is remove in the current mod")?
        {
            return Ok(None);
        };
        if let Some(value) = current_mod
            .get_entry(table, id)
            .chain_err(|| "Impossible to check if an element is changed/added in the current mod")?
        {
            return Ok(Some(value.clone()));
        };
        for r#mod in &self.static_mods {
            if r#mod
                .is_removed(table, id)
                .chain_err(|| "Impossible to check if an element is removed in a static mod")?
            {
                return Ok(None);
            };
            if let Some(value) = r#mod.get_entry(table, id).chain_err(|| {
                "Impossible to check if an element is added/modified by a static mod"
            })? {
                return Ok(Some(value.clone()));
            }
        }
        if let Some(value) = self
            .game
            .base_mod()
            .get_entry(table, id)
            .chain_err(|| "Impossible to check if an element is added by the game mod")?
        {
            return Ok(Some(value.clone()));
        }
        Ok(None)
    }

    pub fn set_entry(&mut self, table: String, id: ID, entry: Entry) -> Result<()> {
        let mut current_mod = self.current_mod.lock().unwrap();
        Ok(current_mod.insert(table, id, entry)?)
    }

    //TASK: add a test for ModPack::remove
    pub fn remove(&mut self, table: String, id: ID) -> Result<()> {
        let mut current_mod = self.current_mod.lock().unwrap();
        Ok(current_mod.remove(table, id)?)
    }
}

#[test]
fn test_modpack() {
    use super::DefaultMod;

    use super::EntryData;
    use super::EntryType;
    use super::EntryValue;
    use super::Metadata;

    use super::TableDataMap;
    use super::ID;
    use crate::builder::DefaultModBuilder;
    use crate::builder::EntryBuilder;
    use crate::builder::TableDataBuilder;
    use crate::builder::TableDataMapBuilder;

    // create a simple game
    /// a simple game, implement the following:
    /// a table named chara: "name": String, "pv": Unsigned64
    /// a table named attack: "name": String, "dmg": Unsigned64
    ///
    /// It also create the following entry:
    /// in char:
    /// "hero": "Soren", 300
    /// "partner": "Twilight", 500
    /// in attack:
    /// "bc": "battle claw", 90
    struct TestGame {
        tabledatamap: Arc<TableDataMap>,
        basemod: Arc<DefaultMod>,
    }
    impl TestGame {
        fn new() -> TestGame {
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

            let chara_tabledata = &tabledatamap["chara".into()];
            let attack_tabledata = &tabledatamap["attack".into()];
            let chara_name_id = chara_tabledata.string_to_id("name".into()).unwrap();
            let chara_pv_id = chara_tabledata.string_to_id("pv".into()).unwrap();

            let basemod = DefaultModBuilder::new(Metadata::default(), tabledatamap.clone())
                .insert(
                    "chara".into(),
                    ID::String("hero".into()),
                    EntryBuilder::new(&chara_tabledata)
                        .set_key(chara_name_id, EntryValue::String("Soren".into()))
                        .set_key(chara_pv_id, EntryValue::Unsigned64(300))
                        .unwrap(),
                )
                .insert(
                    "chara".into(),
                    ID::String("partner".into()),
                    EntryBuilder::new(&chara_tabledata)
                        .set_key(chara_name_id, EntryValue::String("Twilight".into()))
                        .set_key(chara_pv_id, EntryValue::Unsigned64(100))
                        .unwrap(),
                )
                .insert(
                    "attack".into(),
                    ID::String("bc".into()),
                    EntryBuilder::new(&attack_tabledata)
                        .set_key_by_string("name".into(), EntryValue::String("battle claw".into()))
                        .set_key_by_string("dmg".into(), EntryValue::Unsigned64(90))
                        .unwrap(),
                )
                .unwrap();

            TestGame {
                tabledatamap,
                basemod: Arc::new(basemod),
            }
        }
    }
    impl Game for TestGame {
        fn base_mod(&self) -> Arc<dyn ModRead> {
            self.basemod.clone()
        }
        fn get_tabledatamap(&self) -> Arc<TableDataMap> {
            self.tabledatamap.clone()
        }
    }

    let game = Arc::new(TestGame::new());

    let current_mod = Arc::new(Mutex::new(
        DefaultModBuilder::new(Metadata::default(), game.get_tabledatamap()).unwrap(),
    ));

    let static_mod_1 = Arc::new(
        DefaultModBuilder::new(Metadata::default(), game.get_tabledatamap())
            .insert(
                "attack".into(),
                ID::String("ice_shard".into()),
                EntryBuilder::new(&game.get_tabledatamap().get("attack".into()).unwrap())
                    .set_key_by_string("name".into(), EntryValue::String("ice shard".into()))
                    .unwrap(),
            )
            .remove("attack".into(), ID::String("bc".into()))
            .unwrap(),
    );

    let mut modpack = ModPack::new(game.clone(), current_mod);
    modpack.insert_mod(static_mod_1);

    assert_eq!(
        modpack
            .get_entry("chara".into(), &ID::String("hero".into()))
            .unwrap()
            .unwrap()
            .get_key_by_string(
                &game.get_tabledatamap().get("chara".into()).unwrap(),
                "name".into()
            )
            .unwrap()
            .get_string()
            .unwrap(),
        &String::from("Soren")
    );

    assert!(modpack
        .get_entry("attack".into(), &ID::String("bc".into()))
        .unwrap()
        .is_none());

    modpack
        .set_entry(
            "attack".into(),
            ID::String("bc".into()),
            EntryBuilder::new(&game.get_tabledatamap().get("attack".into()).unwrap())
                .set_key_by_string("name".into(), EntryValue::String("battle claw".into()))
                .unwrap(),
        )
        .unwrap();

    assert_eq!(
        modpack
            .get_entry("attack".into(), &ID::String("bc".into()))
            .unwrap()
            .unwrap()
            .get_key_by_string(
                &game.get_tabledatamap().get("attack".into()).unwrap(),
                "name".into()
            )
            .unwrap()
            .get_string()
            .unwrap(),
        &String::from("battle claw")
    );
}
