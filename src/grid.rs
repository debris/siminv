use bevy::{platform::collections::{HashMap, HashSet}, prelude::*};
use crate::{inventory::{Index, InventoryData}, item::Tag, slot::Slot, slot_background::SlotBackground};

pub struct GridInventoryLayout {
    pub columns: usize,
    pub rows: usize,
    pub required_tags: HashMap<Index, Tag>,
    pub blocked_indexes: HashSet<Index>,
}

pub struct GridInventoryConfig {
    pub columns: usize,
    pub rows: usize,
    pub column_gap: Val,
    pub row_gap: Val,
    pub slot_width: Val,
    pub slot_height: Val,
    pub width: Val,
    pub height: Val,
    pub required_tags: HashMap<Index, Tag>,
    // indexes that are not to be displayed as slots
    pub blocked_indexes: HashSet<Index>,
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
            required_tags: HashMap::new(),
            blocked_indexes: HashSet::new(),
        }
    }
}

pub fn build_grid_inventory<T: Bundle + Default>(
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
                .filter_map(move |(x, y)| {
                    let index = Index::new(x as u32, y as u32);
                    if config.blocked_indexes.contains(&index) {
                        return None
                        //return build_null_slot()
                    }
                    Some(index)
                })
                .map(|index| {
                    let size = Val2::new(config.slot_width, config.slot_height);
                    let slot = Slot {
                        item: data.get(&index).copied(),
                        required_tag: config.required_tags.get(&index).cloned(),
                    };
                    build_slot_with_background::<T>(size, slot, index)
                })
                .collect::<Vec<_>>()
                .into_iter()
        ))
    )
}


pub fn build_slot_with_background<T: Bundle + Default>(size: Val2, slot: Slot, index: Index) -> impl Bundle {
    (
        // a wrapper to position a slot in the center of the grid cell
        // we need it so when the user grabs a cell, there is something underneath
        Node {
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            width: size.x,
            height: size.y,
            grid_column: GridPlacement::start(index.x as i16 + 1),
            grid_row: GridPlacement::start(index.y as i16 + 1),
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
            slot,
            T::default(),
            index,
        )
        ],
    )
}

