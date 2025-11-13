use bevy::prelude::*;

use crate::{inventory::Inventories, item::Items, slot, grid};

pub struct ArmouryPlugin;

impl Plugin for ArmouryPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Items>()
            .init_resource::<Inventories>()
            .add_observer(slot::on_add)
            .add_observer(slot::on_pointer_over)
            .add_observer(slot::on_pointer_out)
            .add_observer(slot::on_pointer_drag_start)
            .add_observer(slot::on_pointer_drag)
            .add_observer(slot::on_pointer_drag_end)
            .add_observer(slot::on_pointer_drag_drop)
            .add_systems(Update, slot::update_slot)
            .add_observer(grid::on_add);
    }
}

