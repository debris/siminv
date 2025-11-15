use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{event::{SlotEvent, SlotUpdate}, item::ItemId, slot::{InventoryHandle, Slot}};

#[derive(Component, Deref, DerefMut, Hash, PartialEq, Clone, Copy, Eq, Debug, Deserialize, Serialize)]
pub struct Index(pub UVec2);

impl Index {
    pub fn new(x: u32, y: u32) -> Self {
        Self(UVec2::new(x, y))
    }
}

impl From<(u32, u32)> for Index {
    fn from(value: (u32, u32)) -> Self {
        Index(UVec2::from(value))
    }
}

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
    by_index: HashMap<Index, ItemId>,
    max_size: UVec2,
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

    pub fn remove(&mut self, index: &Index) {
        self.by_index.remove(index);
    }

    // insert into first available slot
    // TODO: fix the default max_size
    pub fn insert(&mut self, item: ItemId) -> Option<Index> {
        for y in 0..self.max_size.y {
            for x in 0..self.max_size.x {
                let index = Index::new(x, y);
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
    query: Query<(&Slot, &InventoryHandle, &Index)>,
    mut inventories: ResMut<Inventories>,
) {
    let Ok((slot, inventory_handle, index)) = query.get(update.entity) else { return };
    let data = inventories.entry_mut(&inventory_handle.0);
    match slot.item {
        Some(item_id) => {
            data.set(*index, item_id);
        },
        None => {
            data.remove(index);
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
        inventory.entry_mut("main").set(Index::new(0, 1), sword);
    }
}

