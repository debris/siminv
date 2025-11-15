use bevy::prelude::*;

use crate::{double_click, inventory::Inventories, item::Items, shift_click, slot, slot_background};

pub struct SiminvPlugin;

impl Plugin for SiminvPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Items>()
            .init_resource::<Inventories>()
            .init_resource::<slot::Dragged>()
            .init_resource::<double_click::DoubleClick>()
            .init_resource::<shift_click::ShiftClick>()
            .add_observer(slot::on_add)
            .add_observer(slot::on_pointer_over)
            .add_observer(slot::on_pointer_out)
            .add_observer(slot::on_pointer_drag_start)
            .add_observer(slot::on_pointer_drag)
            .add_observer(slot::on_pointer_drag_end)
            .add_observer(slot::on_pointer_drag_drop)
            .add_systems(Update, slot::update_slot)
            .add_observer(slot_background::on_add)
            .add_observer(slot_background::on_pointer_over)
            .add_observer(slot_background::on_pointer_out)
            .add_observer(double_click::on_click::<slot::Slot>)
            .add_systems(Update, double_click::update_time)
            .add_observer(shift_click::on_click::<slot::Slot>)
            .add_systems(Update, shift_click::detect_shift_press);
    }
}

