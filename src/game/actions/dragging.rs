use bevy::prelude::*;

use crate::{
    game::components::{BoxType, Cart, CartCrate, FollowMouse, PhysicsCrate},
    input::MousePosition,
    loader::TextureAssets,
};

pub struct DraggingBox {
    pub box_entity: Option<Entity>,
    pub box_type: Option<BoxType>,
    pub is_front_slot: bool,
}

pub enum StartDraggingItem {
    Cart(Entity),
    PhysicsCrate(Entity),
}

pub struct OnStartDragging {
    pub dragged_entity: StartDraggingItem,
    pub is_front_slot: bool,
}

pub fn start_dragging(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    mut dragging: ResMut<DraggingBox>,
    mut events: EventReader<OnStartDragging>,
    mut carts: Query<(&mut Cart, &Children)>,
    mut child_crates: Query<(&mut Visibility, &CartCrate)>,
    physics_crates: Query<&PhysicsCrate>,
) {
    for event in events.iter() {
        match event.dragged_entity {
            StartDraggingItem::Cart(cart_ent) => {
                let (mut cart, children) = match carts.get_mut(cart_ent) {
                    Ok(cart) => cart,
                    Err(e) => {
                        warn!("get cart entity: {:?}", e);
                        continue;
                    }
                };

                // update the cart and set the dragging item here
                dragging.box_type = if event.is_front_slot {
                    let item = cart.front;
                    cart.front = None;
                    item
                } else {
                    let item = cart.back;
                    cart.back = None;
                    item
                };

                // hide the box that's being dragged
                for child in children.iter() {
                    // not all children are cart crates
                    if let Ok((mut vis, child_crate)) = child_crates.get_mut(*child) {
                        if child_crate.is_front_slot == event.is_front_slot {
                            vis.is_visible = false;
                        }
                    }
                }
            }
            StartDraggingItem::PhysicsCrate(crate_ent) => {
                let physics_crate = match physics_crates.get(crate_ent) {
                    Ok(pc) => pc,
                    Err(e) => {
                        warn!("get physics crate: {:?}", e);
                        continue;
                    }
                };

                dragging.box_type = Some(physics_crate.box_type);
                commands.entity(crate_ent).despawn_recursive();
            }
        }

        if dragging.box_type.is_none() {
            // nothing to drag
            continue;
        }

        dragging.is_front_slot = event.is_front_slot;

        // spawn a box based on the box type and attach it to the mouse
        dragging.box_entity = Some(
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_assets.crates.clone(),
                    sprite: TextureAtlasSprite {
                        index: dragging.box_type.unwrap() as usize,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(FollowMouse)
                .id(),
        );
    }
}

pub fn mouse_follower(
    mouse_pos: Res<MousePosition>,
    mut followers: Query<&mut Transform, With<FollowMouse>>,
) {
    if !mouse_pos.in_screen {
        return;
    }

    for mut follower in followers.iter_mut() {
        follower.translation = mouse_pos.world.extend(7.0);
    }
}
