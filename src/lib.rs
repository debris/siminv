mod auto_move;
mod inventory;
mod slot;
mod slot_background;
mod item;
mod grid;
mod plugin;
mod event;
mod slot_updater;
pub mod input;
pub mod simple_renderer;

pub mod prelude {
    pub use crate::{
        auto_move::*,
        inventory::*,
        slot::*,
        slot_background::*,
        item::*,
        grid::*,
        plugin::*,
        event::*,
    };
}

