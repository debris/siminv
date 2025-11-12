use std::marker::PhantomData;
use bevy::{platform::collections::HashMap, prelude::*};
use serde::Deserialize;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Items>();
    }
}

#[derive(Debug, PartialEq, Hash, Clone, Copy, Eq)]
pub struct ItemTypeId(u64);

impl From<u64> for ItemTypeId {
    fn from(value: u64) -> Self {
        ItemTypeId(value)
        
    }
}

#[derive(Debug, Deserialize)]
pub struct ItemType {
    /// Unique tag that can be used to identify this item type.
    pub tag: String,
    /// Name of the item.
    pub name: String,
    /// Sprite path.
    pub sprite_path: String,
    /// Max quantity of items stacked in a single spot of the inventory.
    pub max_stack_size: u64,
}

#[derive(Debug, PartialEq, Hash, Clone, Copy, Eq)]
pub struct ItemId(u64);

impl From<u64> for ItemId {
    fn from(value: u64) -> Self {
        ItemId(value)
    }
}

pub struct Item {
    /// Unique tag that can be used to identify this item type.
    pub tag: String,
    pub stack_size: u64,
}

/// Generates item ids used in runtime.
struct IdFactory<I> {
    current_id: u64,
    id_type: PhantomData<I>,
}

impl<I> Default for IdFactory<I> {
    fn default() -> Self {
        IdFactory { 
            current_id: 0,
            id_type: PhantomData,
        }
    }
}

impl<I: From<u64>> IdFactory<I> {
    pub fn next_id(&mut self) -> I {
        let result = self.current_id;
        self.current_id += 1;
        result.into()
    }
}

#[derive(Resource, Default)]
pub struct Items {
    item_type_ids: IdFactory<ItemTypeId>,
    item_ids: IdFactory<ItemId>,
    item_types: HashMap<ItemTypeId, ItemType>,
    item_types_by_tag: HashMap<String, ItemTypeId>,
    items: HashMap<ItemId, Item>,
}

impl Items {
    pub fn register_item_type(&mut self, item_type: ItemType) -> ItemTypeId {
        let id = self.item_type_ids.next_id();
        self.item_types_by_tag.insert(item_type.tag.clone(), id);
        self.item_types.insert(id, item_type);
        id
    }

    pub fn add_item(&mut self, tag: &str) -> ItemId {
        self.add_items(tag, 1)
    }

    pub fn add_items(&mut self, tag: &str, count: u64) -> ItemId {
        let item = Item {
            tag: tag.to_string(),
            stack_size: count,
        };

        let id = self.item_ids.next_id();
        self.items.insert(id, item);
        id
    }

    pub fn get_item(&self, id: ItemId) -> Option<&Item> {
        self.items.get(&id)
    }

    pub fn get_item_type_with_tag(&self, tag: &str) -> Option<&ItemType> {
        self.item_types_by_tag.get(tag)
            .and_then(|type_id| self.item_types.get(type_id))
    }

    pub fn get_item_meta(&self, id: ItemId) -> Option<(&Item, &ItemType)> {
        self.get_item(id)
            .and_then(|item| self.get_item_type_with_tag(&item.tag).map(|item_type| (item, item_type)))
    }

    /// TODO: return result
    pub fn merge_or_swap(&mut self, item_id: ItemId, into_id: ItemId) -> Option<(Option<ItemId>, Option<ItemId>)> {
        // TODO: convert those to results
        let item = self.items.get(&item_id)?;
        let into = self.items.get(&into_id)?;

        // if they are of different types, just swap them
        if item.tag != into.tag {
            return Some((Some(into_id), Some(item_id)));
        }

        // if they are the same type, check the max stack size
        let item_type = self.get_item_type_with_tag(&item.tag)?;
        // if the max stack size is 1, just swap them
        if item_type.max_stack_size == 1 {
            return Some((Some(into_id), Some(item_id)));
        }

        let max = item_type.max_stack_size;
        let item_count = item.stack_size + into.stack_size;

        if item_count > max {
            //  into has now max stack
            //  item has the rest
            self.items.get_mut(&into_id)?.stack_size = max;
            self.items.get_mut(&item_id)?.stack_size = item_count - max;
            Some((Some(item_id), Some(into_id)))
        } else if item_count == item_type.max_stack_size {
            // into has now max stack
            // item has nothing
            self.items.get_mut(&into_id)?.stack_size = max;
            self.items.remove(&item_id);
            Some((None, Some(into_id)))
        } else {
            // into has now item_count items
            // item has nothing
            self.items.get_mut(&into_id)?.stack_size = item_count;
            self.items.remove(&item_id);
            Some((None, Some(into_id)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_items() {
        let mut items = Items::default();
        items.add_item("sword");
        items.add_item("gloves");
    }
}


