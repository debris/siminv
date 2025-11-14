use auto_move::auto_move;
use bevy::{asset::ron, image::TRANSPARENT_IMAGE_HANDLE, prelude::*};
use double_click::DoubleClick;
use plugin::ArmouryPlugin;
use slot::{Dragged, Slot, SlotHandle};
use event::*;
use inventory::{Inventories, Index};
use item::{ItemType, Items, Tag};
use grid::{GridInventoryConfig, build_grid_inventory};
use bevy_asset_loader::prelude::*;

mod auto_move;
mod double_click;
mod inventory;
mod slot;
mod slot_background;
mod item;
mod grid;
mod plugin;
mod event;
mod palette;
mod tint;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    Next,
}

#[derive(AssetCollection, Resource)]
struct GameAssets {
    #[asset(path = "images/slot_bg.png")]
    slot_background: Handle<Image>,
    #[asset(path = "images/slot_bg_over.png")]
    slot_background_over: Handle<Image>,
    #[asset(path = "images/slot_bg_error.png")]
    slot_background_error: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 11, rows = 22))]
    icons_atlas: Handle<TextureAtlasLayout>,
    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "images/icons.png")]
    icons: Handle<Image>,
}

impl GameAssets {
    fn texture_atlast_for_item(&self, item: &str) -> TextureAtlas {
        let index = match item {
            "sword" => 56,
            "shield" => 67,
            "bow" => 69,
            "helmet" => 78,
            "armor" => 84,
            "stones" => 188,
            // empty spot
            _ => {
                // warn here!
                3
            }
        };

        TextureAtlas::from(self.icons_atlas.clone())
            .with_index(index)
    }
}

fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .load_collection::<GameAssets>()
                .continue_to_state(GameState::Next)
        )

        .add_plugins(ArmouryPlugin)
        
        .add_observer(on_background_add)
        .add_observer(on_background_over)
        .add_observer(on_background_out)

        .add_observer(on_slot_add)
        .add_observer(on_slot_update)

        .add_observer(auto_move::<SlotDoubleClick, Backpack, Equipment>)
        .add_observer(auto_move::<SlotDoubleClick, Equipment, Backpack>)

        .add_systems(OnEnter(GameState::Next), setup)
        .run();
}

#[derive(Component, Default)]
struct Equipment;

#[derive(Component, Default)]
struct Backpack;

fn setup(
    mut commands: Commands,
    mut items: ResMut<Items>,
    mut inventories: ResMut<Inventories>,
) {

    let projection = OrthographicProjection {
        scaling_mode: bevy::camera::ScalingMode::AutoMin { min_width: 800., min_height: 600. },
        ..OrthographicProjection::default_2d()
    };


    commands.spawn((
        Name::new("camera"),
        Camera2d,
        Projection::Orthographic(projection)
    ));
    
    const ITEMS_RON: &str = include_str!("../assets/data/item_types.ron");
    let item_types: Vec<ItemType> = ron::from_str(ITEMS_RON).expect("Failed to parse item_types.ron");

    for item_type in item_types {
        println!("item type: {:?}", item_type);
        items.register_item_type(item_type);
    }

    let shield = items.add_item("shield");
    let helmet = items.add_item("helmet");
    let armor = items.add_item("armor");
    let sword_a = items.add_item("sword");
    let sword_b = items.add_item("sword");
    let bow = items.add_item("bow");
    let stones_a = items.add_items("stones", 5);
    let stones_b = items.add_items("stones", 10);
    let stones_c = items.add_items("stones", 17);

    let data = inventories.entry_mut("main");
    data.set(Index::new(0, 0), shield);
    data.set(Index::new(0, 1), helmet);
    data.set(Index::new(0, 2), armor);
    data.set(Index::new(0, 3), sword_a);
    data.set(Index::new(1, 2), sword_b);
    data.set(Index::new(2, 2), bow);
    data.set(Index::new(3, 0), stones_a);
    data.set(Index::new(3, 1), stones_b);
    data.set(Index::new(3, 2), stones_c);

    
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        BackgroundColor(palette::ColorPair::METAL.light),
        GlobalZIndex(-1),
    ));

    commands.spawn((
        Node {
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Start,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(35),
            height: percent(50),
            ..default()
        },
        children![
            build_grid_inventory::<Backpack>(data, &GridInventoryConfig {
                slot_width: px(80),
                slot_height: px(80),
                columns: 5, 
                rows: 4,
                row_gap: px(0),
                column_gap: px(0),
                width: px(80 * 5),
                height: px(80 * 4),
                ..default()
            })
        ]
    ));

    let data = inventories.entry_mut("eq");

    // |        | helmet |  neckles |
    // | weapon |armor   | off-hand |
    // | ring   |        | ring     |
    // | gloves | boots  | -        |
    
    commands.spawn((
        Node {
            align_self: AlignSelf::Start,
            justify_self: JustifySelf::Start,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(35),
            height: percent(50),
            ..default()
        },
        children![
            build_grid_inventory::<Equipment>(data, &GridInventoryConfig {
                slot_width: px(80),
                slot_height: px(80),
                columns: 3, 
                rows: 4,
                row_gap: px(0),
                column_gap: px(0),
                width: px(80 * 3),
                height: px(80 * 4),
                required_tags: [
                    (Index::new(1, 0), Tag("helmet".into())),
                    (Index::new(2, 0), Tag("neckles".into())),
                    (Index::new(0, 1), Tag("weapon".into())),
                    (Index::new(1, 1), Tag("armor".into())),
                    (Index::new(2, 1), Tag("off-hand".into())),
                    (Index::new(0, 2), Tag("ring".into())),
                    (Index::new(2, 2), Tag("ring".into())),
                    (Index::new(0, 3), Tag("gloves".into())),
                    (Index::new(1, 3), Tag("boots".into())),
                ].into(),
                blocked_indexes: vec![
                    Index::new(0, 0),
                    Index::new(1, 2),
                    Index::new(2, 3),
                ].into_iter().collect(),
                ..default()
            })
        ]
    ));

}

#[derive(Component)]
struct SlotBackgroundImageHandle(Entity);

fn on_background_add(
    add: On<SlotEvent<SlotBackgroundAdd>>,
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    let image = ImageNode::new(assets.slot_background.clone());

    let id = commands.spawn((
        image,
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            ..default()
        }
    )).id();

    commands.entity(add.entity)
        .try_insert(SlotBackgroundImageHandle(id))
        .add_child(id);
}

fn on_background_over(
    over: On<SlotEvent<SlotBackgroundOver>>,
    query_handle: Query<(&SlotBackgroundImageHandle, &SlotHandle)>,
    mut query_image: Query<&mut ImageNode>,
    query_slot: Query<&Slot>,
    assets: Res<GameAssets>,
    items: Res<Items>,
    dragged: Res<Dragged>,
) {
    let Ok((image_handle, slot_handle)) = query_handle.get(over.entity) else { return };
    let Ok(mut image) = query_image.get_mut(image_handle.0) else { return };
    match dragged.0 {
        None => {
            // nothing is dragged
            image.image = assets.slot_background_over.clone();
        },
        Some(item) => {
            let Some(item) = items.get_item_meta(item) else { return };
            // TODO: throw error? crash? 
            let Ok(slot) = query_slot.get(slot_handle.0) else { return };

            if slot.matching_tag(item.tags) {
                image.image = assets.slot_background_over.clone();
            } else {
                // tags are not matching
                image.image = assets.slot_background_error.clone();
            }
        },
    }
}

fn on_background_out(
    out: On<SlotEvent<SlotBackgroundOut>>,
    query_handle: Query<&SlotBackgroundImageHandle>,
    mut query: Query<&mut ImageNode>,
    assets: Res<GameAssets>,
) {
    let Ok(handle) = query_handle.get(out.entity) else { return };
    let Ok(mut image) = query.get_mut(handle.0) else { return };
    image.image = assets.slot_background.clone();
}

#[derive(Component)]
struct SlotItemImageHandle(Entity);

#[derive(Component)]
struct SlotTextHandle(Entity);

fn on_slot_add(
    add: On<SlotEvent<SlotAdd>>,
    mut commands: Commands,
) {
    let image_id = commands.spawn((
        ImageNode::default(),
        Node {
            position_type: PositionType::Absolute,
            width: px(48),
            height: px(48),
            ..default()
        },
        Pickable::IGNORE,
    )).id();

    let text_id = commands.spawn((
        Text::default(),
        TextFont {
            font_size: 12.,
            ..default()
        },
        Node {
            align_self: AlignSelf::End,
            padding: UiRect::bottom(px(6)),
            ..default()
        },
        Pickable::IGNORE,
    )).id();

    commands.entity(add.entity)
        .try_insert((
            SlotItemImageHandle(image_id),
            SlotTextHandle(text_id),
        ))
        .add_children(&[image_id, text_id]);
}

fn on_slot_update(
    update: On<SlotEvent<SlotUpdate>>,
    query_handle: Query<(&SlotItemImageHandle, &SlotTextHandle)>,
    mut query_image: Query<&mut ImageNode>,
    mut query_text: Query<&mut Text>,
    assets: Res<GameAssets>,
    items: Res<Items>,
) {
    let Ok((image_handle, text_handle)) = query_handle.get(update.entity) else { return };
    let Ok(mut image) = query_image.get_mut(image_handle.0) else { return };
    let Ok(mut text) = query_text.get_mut(text_handle.0) else { return };

    match update.item {
        Some(item_id) => {
            let meta = items.get_item_meta(item_id).expect("to be there");
            *image = ImageNode::from_atlas_image(
                assets.icons.clone(),
                assets.texture_atlast_for_item(meta.type_name)
            );
            // if max_stack_size != 1, display max number of elements
            text.0 = match meta.max_stack_size {
                1 => "".to_owned(),
                _ => format!("{}/{}", meta.stack_size, meta.max_stack_size),
            };

        },
        None => {
            image.image = TRANSPARENT_IMAGE_HANDLE;
            // if there's no item, maybe write a placeholder spot
            text.0 = match update.required_tag {
                None => "".to_owned(),
                Some(Tag(ref tag)) => {
                    println!("tagging: {}", tag);
                    format!("[{}]", tag)
                }
            }
        }
    }
}

