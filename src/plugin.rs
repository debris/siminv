use bevy::prelude::*;

use crate::{inventory::{self}, slot, slot_background, slot_updater, input};

pub struct SiminvPlugin;

impl Plugin for SiminvPlugin {
    fn build(&self, app: &mut App) {
        app
            // double click
            .init_resource::<input::double_click::DoubleClick>()
            .add_observer(input::double_click::on_click::<slot::Slot>)
            .add_systems(Update, input::double_click::update_time)

            // shift click
            .init_resource::<input::shift_click::ShiftClick>()
            .add_observer(input::shift_click::on_click::<slot::Slot>)
            .add_systems(Update, input::shift_click::detect_shift_press)

            // long hover
            .init_resource::<input::hover::Hover>()
            .add_observer(input::hover::on_over::<slot_background::SlotBackground>)
            .add_observer(input::hover::on_out::<slot_background::SlotBackground>)
            .add_systems(Update, input::hover::update_time)

            .init_resource::<slot::Dragged>()
            .init_resource::<slot_updater::SlotUpdater>()
            .add_observer(slot::on_add)
            .add_observer(slot::on_pointer_over)
            .add_observer(slot::on_pointer_out)
            .add_observer(slot::on_pointer_drag_start)
            .add_observer(slot::on_pointer_drag)
            .add_observer(slot::on_pointer_drag_end)
            .add_observer(slot::on_pointer_drag_drop)
            .add_observer(slot_background::on_add)
            .add_observer(slot_background::on_pointer_over)
            .add_observer(slot_background::on_pointer_out)
            .add_observer(inventory::on_slot_update)
            .add_observer(slot_updater::on_slot_add)
            .add_systems(Update, slot_updater::propagete_inventory_changes);
    }
}

