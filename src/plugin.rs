use bevy::prelude::*;

use crate::{double_click, inventory::Inventories, item::Items, slot, slot_background};

pub struct ArmouryPlugin;

impl Plugin for ArmouryPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Items>()
            .init_resource::<Inventories>()
            .init_resource::<slot::Dragged>()
            .init_resource::<double_click::DoubleClick>()
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
            .add_observer(double_click::on_click)
            .add_systems(Update, double_click::update_time);
    }
}

