use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{event::{SlotEvent, SlotUpdate}, item::ItemId, slot::{InventoryHandle, Slot}};

// TODO: consider a different data structure that HashMap<String, _> and HashMap<UVec2, _> 
// for storing and fast access to data
#[derive(Resource, Default, Deserialize, Serialize)]
pub struct Inventory {
    collections_by_name: HashMap<String, InventoryCollection>,
    // a list of indexes that should be updated by the ui
    #[serde(skip)]
    modified: Vec<(String, UVec2)>,
}

impl Inventory {
    pub fn get(&self, collection: &str, index: &UVec2) -> Option<&ItemId> {
        self.collections_by_name
            .get(collection)
            .and_then(|collection| collection.get(&index))
    }

    pub fn set(&mut self, collection: &str, index: UVec2, item: ItemId) {
        self.set_unregistered(collection, index, item);
        self.modified.push((collection.to_string(), index));
    }

    fn set_unregistered(&mut self, collection: &str, index: UVec2, item: ItemId) {
        self.collections_by_name
            .entry(collection.to_string())
            .or_default()
            .set(index, item);
    }

    pub fn remove(&mut self, collection: &str, index: UVec2) {
        self.remove_unregistered(collection, index);
        self.modified.push((collection.to_string(), index));
    }

    fn remove_unregistered(&mut self, collection: &str, index: UVec2) {
        self.collections_by_name
            .get_mut(&collection.to_string())
            .map(|collection| collection.remove(&index));
    }

    pub fn set_max_size(&mut self, collection: &str, max_size: UVec2) {
        self.collections_by_name
            .entry(collection.to_string())
            .or_default()
            .max_size = max_size;
    }

    pub fn add(&mut self, collection: &str, item: ItemId) -> Option<UVec2> {
        let maybe_added = self.collections_by_name
            .entry(collection.to_string())
            .or_default()
            .add(item);

        if let Some(added) = maybe_added {
            self.modified.push((collection.to_string(), added));
        }

        maybe_added
    }

    pub(crate) fn take_modified(&mut self) -> Vec<(String, UVec2)> {
        let mut result = Vec::new();
        core::mem::swap(&mut self.modified, &mut result);
        result
    }
}

#[derive(Deserialize, Serialize)]
struct InventoryCollection {
    by_index: HashMap<UVec2, ItemId>,
    max_size: UVec2,
}

impl Default for InventoryCollection {
    fn default() -> Self {
        Self {
            by_index: HashMap::default(),
            // inserts everything into 1 row
            max_size: UVec2::new(u32::MAX, 1),
        }
    }
}

impl InventoryCollection {
    fn set(&mut self, index: UVec2, item: ItemId) {
        self.by_index.insert(index, item);
    }

    fn get(&self, index: &UVec2) -> Option<&ItemId> {
        self.by_index.get(index)
    }
    
    fn get_mut(&mut self, index: &UVec2) -> Option<&mut ItemId> {
        self.by_index.get_mut(index)
    }

    fn remove(&mut self, index: &UVec2) {
        self.by_index.remove(index);
    }

    // insert into first available slot
    // returns None if the collection is full
    // this may take a while?
    fn add(&mut self, item: ItemId) -> Option<UVec2> {
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
    mut inventory: ResMut<Inventory>,
) {
    let Ok((slot, inventory_handle)) = query.get(update.entity) else { return };
    match slot.item {
        Some(item_id) => {
            inventory.set_unregistered(&inventory_handle.collection, inventory_handle.index, item_id);
        },
        None => {
            inventory.remove_unregistered(&inventory_handle.collection, inventory_handle.index);
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


        let mut inventory = Inventory::default();
        inventory.add("main", sword);
    }
}

