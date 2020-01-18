use super::Entry;
use super::Metadata;
use super::TableDataMap;
use super::ID;
use crate::errors::*;
use std::collections::BTreeSet;
use std::sync::Arc;

/// A mod (A content that modify a game). See the defintion file for this
pub trait ModRead {
    /// Return the [MetaData] of this mod
    fn get_metadata(&self) -> &Metadata;
    /// Return the [Game] this mod modify
    fn get_tabledatamap(&self) -> Arc<TableDataMap>;
    /// Return the list of table in which this mod add/modifify/delete elements
    fn get_modified_table_list(&self) -> Vec<String>;
    /// Return the list of entry in a table of this mod that was modified/added by it
    ///
    /// Return an empty vector if nothing is modified/added
    fn get_modified_entry_list(&self, table: &str) -> Result<Vec<ID>>;
    /// Return an entry of a mod (only if it is modified/added by it)
    fn get_entry(&self, table: &str, id: &ID) -> Result<Option<&Entry>>;
    /// Return true if this value is marked as deleted by this mod
    fn is_removed(&self, table: &str, id: &ID) -> Result<bool>; //TASK: default impl from list_removed
    /// list removed element in a set
    ///
    /// Return an empty set if none is removed by this mod
    fn list_removed(&self, table: &str) -> Result<BTreeSet<ID>>;
}

/// A Mod that can be modified
pub trait ModWrite: ModRead {
    /// Set/modify an entry of the mod
    fn insert(&mut self, table: String, id: ID, value: Entry) -> Result<()>;
    /// Mark a value as removed
    fn remove(&mut self, table: String, id: ID) -> Result<()>;
    /// remove the mark of a deleted id
    fn restore(&mut self, table: &str, id: &ID) -> Result<()>;
}
