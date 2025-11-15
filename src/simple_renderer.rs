use std::marker::PhantomData;

use bevy::{image::TRANSPARENT_IMAGE_HANDLE, prelude::*};

use crate::{event::{SlotAdd, SlotBackgroundAdd, SlotBackgroundOut, SlotBackgroundOver, SlotEvent, SlotUpdate}, item::{Items, Tag}, slot::{Dragged, Slot, SlotHandle}};

#[derive(Debug)]
pub enum SimpleImageHandle {
    Direct(Handle<Image>),
    AtlasImage(Handle<Image>, TextureAtlas),
}

impl From<SimpleImageHandle> for ImageNode {
    fn from(value: SimpleImageHandle) -> Self {
        match value {
            SimpleImageHandle::Direct(image) => ImageNode::new(image),
            SimpleImageHandle::AtlasImage(image, atlas) => ImageNode::from_atlas_image(image, atlas),
        }
    }
}

pub trait SimpleRendererAssets: Resource {
    fn background(&self) -> SimpleImageHandle;
    fn background_over(&self) -> SimpleImageHandle;
    fn background_error(&self) -> SimpleImageHandle;
    fn item(&self, item: &str) -> SimpleImageHandle;
}

/// Renders all slots with marker S  with assets T.
#[derive(Debug)]
pub struct SiminvSimpleRendererPlugin<T, S> {
    _assets_marker: PhantomData<T>,
    _slots_marker: PhantomData<S>,
}

impl<T, S> Default for SiminvSimpleRendererPlugin<T, S> {
    fn default() -> Self {
        Self {
            _assets_marker: PhantomData::default(),
            _slots_marker: PhantomData::default(),
        }
    }
}

impl<T, S> Plugin for SiminvSimpleRendererPlugin<T, S> where T: SimpleRendererAssets, S: Component {
    fn build(&self, app: &mut App) {
        println!("siming ren");
        app
            .add_observer(on_background_add::<T, S>)
            .add_observer(on_background_over::<T, S>)
            .add_observer(on_background_out::<T, S>)
            .add_observer(on_slot_add::<S>)
            .add_observer(on_slot_update::<T, S>);
        // TODO:
    }
}

#[derive(Component)]
struct SlotBackgroundImageHandle(Entity);

fn on_background_add<T: SimpleRendererAssets, S: Component>(
    add: On<SlotEvent<SlotBackgroundAdd>, S>,
    mut commands: Commands,
    assets: Res<T>,
) {
    println!("bg add");
    let image: ImageNode = assets.background().into();

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

fn on_background_over<T: SimpleRendererAssets, S: Component>(
    over: On<SlotEvent<SlotBackgroundOver>, S>,
    query_handle: Query<(&SlotBackgroundImageHandle, &SlotHandle)>,
    mut query_image: Query<&mut ImageNode>,
    query_slot: Query<&Slot>,
    assets: Res<T>,
    items: Res<Items>,
    dragged: Res<Dragged>,
) {
    println!("bg over");
    let Ok((image_handle, slot_handle)) = query_handle.get(over.entity) else { return };
    let Ok(mut image) = query_image.get_mut(image_handle.0) else { return };
    match dragged.0 {
        None => {
            // nothing is dragged
            *image = assets.background_over().into();
        },
        Some(item) => {
            let Some(item) = items.get_item_meta(item) else { return };
            // TODO: throw error? crash? 
            let Ok(slot) = query_slot.get(slot_handle.0) else { return };

            if slot.matching_tag(item.tags) {
                *image = assets.background_over().into();
            } else {
                // tags are not matching
                *image = assets.background_error().into();
            }
        },
    }
}

fn on_background_out<T: SimpleRendererAssets, S: Component>(
    out: On<SlotEvent<SlotBackgroundOut>, S>,
    query_handle: Query<&SlotBackgroundImageHandle>,
    mut query: Query<&mut ImageNode>,
    assets: Res<T>,
) {
    let Ok(handle) = query_handle.get(out.entity) else { return };
    let Ok(mut image) = query.get_mut(handle.0) else { return };
    *image = assets.background().into();
}

#[derive(Component)]
struct SlotItemImageHandle(Entity);

#[derive(Component)]
struct SlotTextHandle(Entity);

fn on_slot_add<S: Component>(
    add: On<SlotEvent<SlotAdd>, S>,
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

fn on_slot_update<T: SimpleRendererAssets, S: Component>(
    update: On<SlotEvent<SlotUpdate>, S>,
    query_handle: Query<(&SlotItemImageHandle, &SlotTextHandle)>,
    mut query_image: Query<&mut ImageNode>,
    mut query_text: Query<&mut Text>,
    assets: Res<T>,
    items: Res<Items>,
) {
    let Ok((image_handle, text_handle)) = query_handle.get(update.entity) else { return };
    let Ok(mut image) = query_image.get_mut(image_handle.0) else { return };
    let Ok(mut text) = query_text.get_mut(text_handle.0) else { return };

    match update.item {
        Some(item_id) => {
            let meta = items.get_item_meta(item_id).expect("to be there");
            *image = assets.item(meta.type_name).into();
            
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

