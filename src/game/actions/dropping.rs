use bevy::prelude::*;

use crate::game::components::{Cart, CartCrate, FollowMouse, Ship};

use super::dragging::DraggingBox;

#[derive(Debug)]
pub struct ShipSlots {
    pub slots: [Option<Entity>; 3],
}

impl Default for ShipSlots {
    fn default() -> Self {
        Self {
            slots: [None, None, None],
        }
    }
}

pub struct OnDropCrate {
    pub ship: Option<Entity>,
}

pub fn handle_drop(
    mut commands: Commands,
    mut events: EventReader<OnDropCrate>,
    mut dragging: ResMut<DraggingBox>,
    followers: Query<Entity, With<FollowMouse>>,
    mut carts: Query<(&mut Cart, &Children)>,
    mut child_crates: Query<(&mut Visibility, &CartCrate)>,
    mut ships: Query<&mut Ship>,
) {
    let mut done = false;
    for evt in events.iter() {
        if done {
            warn!("Repeat drop event being ignored");
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

        // set the cart crate to visible again, or add it to the ship if it was dropped on a ship
        match evt.ship {
            Some(ship_ent) => {
                let mut ship = ships
                    .get_mut(ship_ent)
                    .expect("Should be able to find ship");
                ship.crates.push(dragging.box_type.unwrap())
            }
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

        // reset the dragging resources
        if dragging.is_front_slot {
            cart.front = dragging.box_type;
        } else {
            cart.back = dragging.box_type;
        }

        dragging.box_type = None;
        dragging.cart_entity = None;
    }
}
