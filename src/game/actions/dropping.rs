use bevy::prelude::*;

use crate::game::components::{Cart, CartCrate, FollowMouse, ShipHold};

use super::dragging::DraggingBox;

#[derive(Clone, Copy, Debug)]
pub enum ShipSlotType {
    Empty,
    Arriving(Entity),
    Occupied(Entity),
}

impl Default for ShipSlotType {
    fn default() -> Self {
        ShipSlotType::Empty
    }
}

#[derive(Default, Debug)]
pub struct ShipSlots {
    pub slots: [ShipSlotType; 3],
}

pub struct OnDropCrate {
    pub ship: Option<Entity>,
}

pub struct OnCrateDroppedOnShip(pub Entity);

pub fn handle_drop(
    mut commands: Commands,
    mut on_drop_events: EventReader<OnDropCrate>,
    mut on_drop_on_ship_events: EventWriter<OnCrateDroppedOnShip>,
    mut dragging: ResMut<DraggingBox>,
    followers: Query<Entity, With<FollowMouse>>,
    mut carts: Query<(&mut Cart, &Children)>,
    mut child_crates: Query<(&mut Visibility, &CartCrate)>,
    mut ships: Query<&mut ShipHold>,
) {
    let mut done = false;
    for evt in on_drop_events.iter() {
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
        let dropped_on_ship = match evt.ship {
            Some(ship_ent) => {
                let mut ship_hold = ships
                    .get_mut(ship_ent)
                    .expect("Should be able to find ship");
                ship_hold.accept_crate(dragging.box_type.unwrap());
                on_drop_on_ship_events.send(OnCrateDroppedOnShip(ship_ent));
                true
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

                false
            }
        };

        // reset the dragging resources
        if dragging.is_front_slot {
            cart.front = if dropped_on_ship {
                None
            } else {
                dragging.box_type
            };
        } else {
            cart.back = if dropped_on_ship {
                None
            } else {
                dragging.box_type
            };
        }

        dragging.box_type = None;
        dragging.cart_entity = None;
    }
}
