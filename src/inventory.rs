use bevy::{platform::collections::HashMap, prelude::*};

use crate::{inventory_ui::Index, item::ItemId};

#[derive(Resource, Default)]
pub struct Inventories {
    inventories_by_name: HashMap<String, InventoryData>,
}

impl Inventories {
    pub fn data(&self, name: &str) -> Option<&InventoryData> {
        self.inventories_by_name.get(name)
    }

    pub fn entry_mut(&mut self, name: &str) -> &mut InventoryData {
        self.inventories_by_name.entry(name.to_owned()).or_insert_with(Default::default)
    }
}

#[derive(Default)]
pub struct InventoryData {
    by_index: HashMap<Index, ItemId>
}

impl InventoryData {
    pub fn set(&mut self, index: Index, item: ItemId) {
        self.by_index.insert(index, item);
    }

    pub fn get(&self, index: &Index) -> Option<&ItemId> {
        self.by_index.get(index)
    }

    pub fn get_mut(&mut self, index: &Index) -> Option<&mut ItemId> {
        self.by_index.get_mut(index)
    }
}

#[cfg(test)]
mod test {
    use crate::item::Items;
    use super::*;

    #[test]
    fn test_inventory() {
        let mut items = Items::default();
        let sword = items.add_item("sword");


        let mut inventory = Inventories::default();
        inventory.data_mut("main").set(Index::new(0, 1), sword);
    }
}

