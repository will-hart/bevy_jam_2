use bevy::prelude::*;

use crate::{
    game::components::{BoxType, Cart, CartCrate, FollowMouse},
    input::MousePosition,
    loader::TextureAssets,
};

pub struct DraggingBox {
    pub cart_entity: Option<Entity>,
    pub box_type: Option<BoxType>,
    pub is_front_slot: bool,
}

pub struct StartDragging {
    pub cart_entity: Entity,
    pub is_front_slot: bool,
}
pub struct StopDragging {
    pub ship: Option<Entity>,
}

pub fn start_dragging(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    mut dragging: ResMut<DraggingBox>,
    mut events: EventReader<StartDragging>,
    mut carts: Query<(&mut Cart, &Children)>,
    mut child_crates: Query<(&mut Visibility, &CartCrate)>,
) {
    for event in events.iter() {
        // find the cart that we're dragging from
        let (mut cart, children) = match carts.get_mut(event.cart_entity) {
            Ok(cart) => cart,
            Err(e) => {
                warn!("{:?}", e);
                continue;
            }
        };

        // update the cart and set the dragging item here
        dragging.cart_entity = Some(event.cart_entity);
        dragging.box_type = if event.is_front_slot {
            let item = cart.front.clone();
            cart.front = None;
            item
        } else {
            let item = cart.back.clone();
            cart.back = None;
            item
        };
        dragging.is_front_slot = event.is_front_slot;

        // hide the box that's being dragged
        for child in children.iter() {
            let (mut vis, child_crate) =
                child_crates.get_mut(*child).expect("should have children");
            if child_crate.is_front_slot == event.is_front_slot {
                vis.is_visible = false;
            }
        }

        // spawn a box based on the box type and attach it to the mouse
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_assets.crates.clone(),
                sprite: TextureAtlasSprite {
                    index: dragging.box_type.unwrap() as usize,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(FollowMouse);
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

pub fn stop_dragging(
    mut commands: Commands,
    mut events: EventReader<StopDragging>,
    mut dragging: ResMut<DraggingBox>,
    followers: Query<Entity, With<FollowMouse>>,
    mut carts: Query<(&mut Cart, &Children)>,
    mut child_crates: Query<(&mut Visibility, &CartCrate)>,
) {
    let mut done = false;
    for evt in events.iter() {
        if done {
            continue;
        }

        // despawn followers
        done = true;
        for follower in followers.iter() {
            commands.entity(follower).despawn();
        }

        // redo the cart
        let (mut cart, children) = match carts.get_mut(dragging.cart_entity.unwrap()) {
            Ok(cv) => cv,
            Err(_) => {
                warn!("Can't find card in stop_dragging, aborting");
                return;
            }
        };

        // reset the dragging resources
        if dragging.is_front_slot {
            cart.front = dragging.box_type;
        } else {
            cart.back = dragging.box_type;
        }

        dragging.box_type = None;
        dragging.cart_entity = None;

        // set the cart crate to visible again
        match evt.ship {
            Some(ship) => todo!(),
            None => {
                // "drop back" on the cart
                for child in children.iter() {
                    let (mut vis, child_crate) =
                        child_crates.get_mut(*child).expect("should have children");
                    if child_crate.is_front_slot == dragging.is_front_slot {
                        vis.is_visible = true;
                    }
                }
            }
        }
    }
}
