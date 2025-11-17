use bevy::{platform::collections::{HashMap, HashSet}, prelude::*};
use crate::{item::Tag, slot::{InventoryHandle, Slot}, slot_background::SlotBackground};

/// Defines slot sizes and gaps beetween slots.
pub struct GridStyle {
    pub column_gap: Val,
    pub row_gap: Val,
    pub slot_width: Val,
    pub slot_height: Val,
}

impl Default for GridStyle {
    fn default() -> Self {
        Self {
            slot_width: percent(100),
            slot_height: percent(100),
            column_gap: px(0.),
            row_gap: px(0.),
        }
    }
}

/// Specifies source of the displayed grid and slot requirements.
pub struct GridInventoryConfig<'a> {
    pub collection: &'a str,
    pub columns: usize,
    pub rows: usize,
    pub required_tags: HashMap<UVec2, Tag>,
    /// indexes that are not to be displayed as slots
    pub blocked_indexes: HashSet<UVec2>,
}

impl<'a> Default for GridInventoryConfig<'a> {
    fn default() -> Self {
        Self {
            collection: "",
            columns: 0,
            rows: 0,
            required_tags: HashMap::new(),
            blocked_indexes: HashSet::new(),
        }
    }
}

/// Helper function to build grid Bundle.
pub fn build_grid_inventory<T: Bundle + Default>(
    style: &GridStyle,
    config: &GridInventoryConfig,
) -> impl Bundle {
    (
        Node {
            display: Display::Grid,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            grid_template_columns: RepeatedGridTrack::flex(config.columns as u16, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(config.rows as u16, 1.0),
            column_gap: style.column_gap,
            row_gap: style.row_gap,
            // autoadjust to the size of the content
            width: Val::Auto,
            height: Val::Auto,
            ..default()
        },
        Children::spawn(SpawnIter(
            (0..config.columns)
                .flat_map(move |x| (0..config.rows).map(move |y| (x, y)))
                .filter_map(move |(x, y)| {
                    let index = UVec2::new(x as u32, y as u32);
                    if config.blocked_indexes.contains(&index) {
                        return None
                    }
                    Some(index)
                })
                .map(|index| {
                    let size = Val2::new(style.slot_width, style.slot_height);
                    let mut slot = Slot::empty();
                    slot.required_tag = config.required_tags.get(&index).cloned();
                    build_slot_with_background::<T>(size, slot, index, InventoryHandle {
                        collection: config.collection.to_string(),
                        index,
                    })
                })
                .collect::<Vec<_>>()
                .into_iter()
        ))
    )
}


fn build_slot_with_background<T: Bundle + Default>(size: Val2, slot: Slot, index: UVec2, handle: InventoryHandle) -> impl Bundle {
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
            handle,
        )
        ],
    )
}

