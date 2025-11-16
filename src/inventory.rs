use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{event::{SlotEvent, SlotUpdate}, item::ItemId, slot::{InventoryHandle, Slot}};

#[derive(Resource, Default, Deserialize, Serialize)]
pub struct Inventories {
    inventories_by_name: HashMap<String, InventoryData>,
}

impl Inventories {
    pub fn data(&self, name: &str) -> Option<&InventoryData> {
        self.inventories_by_name.get(name)
    }

    pub fn entry_mut(&mut self, name: &str) -> &mut InventoryData {
        self.inventories_by_name.entry(name.to_owned()).or_default()
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct InventoryData {
    by_index: HashMap<UVec2, ItemId>,
    max_size: UVec2,
}

impl InventoryData {
    pub fn set(&mut self, index: UVec2, item: ItemId) {
        self.by_index.insert(index, item);
    }

    pub fn get(&self, index: &UVec2) -> Option<&ItemId> {
        self.by_index.get(index)
    }

    pub fn get_mut(&mut self, index: &UVec2) -> Option<&mut ItemId> {
        self.by_index.get_mut(index)
    }

    pub fn remove(&mut self, index: &UVec2) {
        self.by_index.remove(index);
    }

    // insert into first available slot
    // TODO: fix the default max_size
    pub fn insert(&mut self, item: ItemId) -> Option<UVec2> {
        for y in 0..self.max_size.y {
            for x in 0..self.max_size.x {
                let index = UVec2::new(x, y);
                if !self.by_index.contains_key(&index) {
                    self.by_index.insert(index, item);
                    return Some(index)
                }
            }
        }
        None
    }
}

pub(crate) fn on_slot_update(
    update: On<SlotEvent<SlotUpdate>>,
    query: Query<(&Slot, &InventoryHandle)>,
    mut inventories: ResMut<Inventories>,
) {
    let Ok((slot, inventory_handle)) = query.get(update.entity) else { return };
    let data = inventories.entry_mut(&inventory_handle.collection);
    match slot.item {
        Some(item_id) => {
            data.set(inventory_handle.index, item_id);
        },
        None => {
            data.remove(&inventory_handle.index);
        }
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
        inventory.entry_mut("main").set(UVec2::new(0, 1), sword);
    }
}

