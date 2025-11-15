use bevy::prelude::*;

use crate::{event::SlotEvent, inventory::Index, item::Items, slot::Slot};

pub struct MovePolicy;

impl MovePolicy {
    pub const ONLY_EMPTY: u8 = 0u8;
    pub const EMPTY_OR_REPLACE: u8 = 1u8;
}

// SlotEvent can be double-click, cmd-click, ctrl-click, shift-click etc.
/// This observer function is used to move Items from collection F, to collection T, when
/// event E is triggered.
// on_event_move_from_to
pub fn on_event_move_to<E: Send + Sync + 'static, F: Component, T: Component, const R: u8> (
    event: On<SlotEvent<E>, F>,
    mut query_from: Query<&mut Slot, Without<T>>,
    query_into: Query<(&mut Slot, Option<&Index>), With<T>>,
    items: Res<Items>,
) {
    // slot that triggered the event
    let Ok(mut from_slot) = query_from.get_mut(event.entity) else { return };
    // if there is no item in the slot, ignore event
    let Some(from_item_id) = from_slot.item else { return };
    // this should throw? panic? if not present? 
    let Some(from_item) = items.get_item_meta(from_item_id) else { return };
    
    // TODO:
    let stackable = from_item.max_stack_size != 1;

    // lets order the slots row after row
    let mut ordered_into_slots = query_into
        .into_iter()
        // sort the slots, so we start filling rows, one after another
        .sort_by::<(&Slot, Option<&Index>)>(|(_, index_a), (_, index_b)| {
            // fill items outside of the grid first
            let Some(a) = index_a else { return core::cmp::Ordering::Less };
            let Some(b) = index_b else { return core::cmp::Ordering::Greater };

            if a.y == b.y {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        })
        .map(|(slot, _)| slot)
        .collect::<Vec<_>>();

    let empty_into_slot = ordered_into_slots
        .iter_mut()
        .find(|slot| {
            slot.is_empty() && slot.matching_tag(from_item.tags) 
        });

    match empty_into_slot {
        Some(into_slot) => {
            // TODO: use more sophisticated methods, like merge && swap 
            core::mem::swap(&mut from_slot.item, &mut into_slot.item);
        },
        None => {
            // if there are no empty slots matching the tag, maybe replace the existing slot?
            if R == MovePolicy::EMPTY_OR_REPLACE {
                let Some(into_slot) = ordered_into_slots
                    .iter_mut()
                    .find(|slot| slot.matching_tag(from_item.tags))
                    // if there are no matching slots, just return
                    else {
                        return
                    };
                core::mem::swap(&mut from_slot.item, &mut into_slot.item);
            }
        },
    }
}

