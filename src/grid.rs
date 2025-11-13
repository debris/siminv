use bevy::prelude::*;
use crate::{event::{SlotEvent, TriggerSlotEvent}, inventory::{Index, InventoryData}, slot::Slot, slot_background::SlotBackground};

pub struct GridInventoryConfig {
    pub columns: usize,
    pub rows: usize,
    pub column_gap: Val,
    pub row_gap: Val,
    pub slot_width: Val,
    pub slot_height: Val,
    pub width: Val,
    pub height: Val,
}

impl Default for GridInventoryConfig {
    fn default() -> Self {
        GridInventoryConfig {
            columns: 4,
            rows: 4,
            slot_width: percent(100),
            slot_height: percent(100),
            column_gap: px(10.),
            row_gap: px(10.),
            width: percent(100),
            height: percent(100),
        }
    }
}

pub fn build_grid_inventory<T: Component + Default>(
    data: &InventoryData,
    config: &GridInventoryConfig,
) -> impl Bundle {
    (
        Node {
            display: Display::Grid,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            grid_template_columns: RepeatedGridTrack::flex(config.columns as u16, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(config.rows as u16, 1.0),
            column_gap: config.column_gap,
            row_gap: config.row_gap,
            width: config.width,
            height: config.height,
            ..default()
        },
        Children::spawn(SpawnIter(
            (0..config.columns)
            .flat_map(move |x| (0..config.rows).map(move |y| (x, y)))
            .map(move |(x, y)| { 
                let index = Index::new(x as u32, y as u32);
                (
                    // a wrapper to position a slot in the center of the grid cell
                    // we need it so when the user grabs a cell, there is something underneath
                    Node {
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        width: config.slot_width,
                        height: config.slot_height,
                        ..default()
                    },
                    SlotBackground,
                    T::default(),
                    children![
                    (
                        Node {
                            align_self: AlignSelf::Center,
                            justify_self: JustifySelf::Center,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: percent(100),
                            height: percent(100),
                            ..default()
                        },
                        Slot {
                            item: data.get(&index).copied(),
                            ..default()
                        },
                        T::default(),
                        index,
                    )
                    ],
                )
            // collect so the spawn iter does not keep a lifetime of data && config
            }).collect::<Vec<_>>().into_iter()
            ))
    )
}


