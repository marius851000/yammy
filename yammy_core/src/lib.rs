#![recursion_limit = "1024"] //For error_chain
#[macro_use]
extern crate error_chain;

pub mod errors {
    error_chain! {
        foreign_links {
        }
    }
}

mod r#mod;
pub use r#mod::{ModRead, ModWrite};

///An id that can be used to index an Entry.
#[derive(Debug, PartialEq, Clone, Hash, Eq, PartialOrd, Ord)]
pub enum ID {
    String(String),
    Integer(u64),
}

mod metadata;
pub use metadata::Metadata;

mod entry;
pub use entry::Entry;

mod tabledata;
pub use tabledata::TableData;

mod entryvalue;
pub use entryvalue::EntryValue;

mod entrydata;
pub use entrydata::EntryData;
pub use entrydata::EntryType;

mod tabledatamap;
pub use tabledatamap::TableDataMap;

mod game;
pub use game::Game;

//dead code ATM
//mod idlist;
//pub use idlist::IDList;

mod defaultmod;
pub use defaultmod::DefaultMod;

mod modpack;
pub use modpack::ModPack;

pub mod builder;
