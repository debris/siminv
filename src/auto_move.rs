use bevy::{ecs::query, prelude::*};

use crate::{event::SlotEvent, item::Items, slot::Slot};

// SlotEvent can be double-click, cmd-click, ctrl-click, shift-click etc.
pub fn auto_move<E: Send + Sync + 'static, F: Component, T: Component> (
    event: On<SlotEvent<E>, F>,
    mut query_from: Query<&mut Slot, Without<T>>,
    query_into: Query<&mut Slot, With<T>>,
    items: Res<Items>,
) {
    println!("double click");
    // slot that triggered the event
    let Ok(mut from_slot) = query_from.get_mut(event.entity) else { return };
    // if there is no item in the slot, ignore event
    let Some(from_item_id) = from_slot.item else { return };
    // this should throw? panic? if not present? 
    let Some(from_item) = items.get_item_meta(from_item_id) else { return };
    
    // move it into the first matching slot with marker T
    // the order of the children is guaranteed?, so we should first fill up the first slots
    let Some(mut into_slot) = query_into
        .into_iter()
        .find(|slot| {
            slot.is_empty() && slot.matching_tag(from_item.tags) 
        }) else { 
            // if there are no empty slots matching the tag, do nothing
            return 
        };

    // TODO: use more sophisticated methods, like merge && swap 
    core::mem::swap(&mut from_slot.item, &mut into_slot.item);
}

pub fn test_double(
)
{
}

