use super::ModRead;
use super::TableDataMap;
use std::sync::Arc;

pub trait Game {
    /// Return the base mod for the game
    fn base_mod(&self) -> Arc<dyn ModRead>;
    /// Return the [TableDataMap] of this mod
    fn get_tabledatamap(&self) -> Arc<TableDataMap>;
}
