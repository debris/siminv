use bevy::{platform::collections::HashMap, prelude::*};

use crate::{event::{SlotAdd, SlotEvent, SlotUpdate, TriggerSlotEvent}, prelude::Inventory, slot::{InventoryHandle, Slot}};

#[derive(Resource, Default)]
pub struct SlotUpdater {
    slot_by_inventory_handle: HashMap<InventoryHandle, Entity>,
}

pub fn on_slot_add(
    added: On<SlotEvent<SlotAdd>, Slot>,
    mut query: Query<(&mut Slot, Option<&InventoryHandle>)>,
    inventory: Res<Inventory>,
    mut updater: ResMut<SlotUpdater>,
) {
    let Ok((mut slot, maybe_handle)) = query.get_mut(added.entity) else { return };

    if let Some(handle) = maybe_handle {
        slot.item = inventory.get(&handle.collection, &handle.index).cloned();

        // register slot
        // TODO: unregister at some point
        updater.slot_by_inventory_handle.insert(handle.clone(), added.entity);
    }
}

pub fn propagete_inventory_changes(
    mut inventory: ResMut<Inventory>,
    mut commands: Commands,
    updater: Res<SlotUpdater>,
    mut query: Query<&mut Slot>
) {
    for (collection, index) in inventory.take_modified() {
        let handle = InventoryHandle {
            collection,
            index
        };

        let Some(slot_id) = updater.slot_by_inventory_handle.get(&handle) else { continue };

        let Ok(mut slot) = query.get_mut(*slot_id) else { continue };
        slot.item = inventory.get(&handle.collection, &handle.index).cloned();

        commands.trigger_slot_event(SlotEvent::new(*slot_id, SlotUpdate));
    }
}

