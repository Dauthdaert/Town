use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;
use big_brain::prelude::*;

use crate::{
    ai::characteristics::thirst::Thirst,
    map_gen::{components::WaterSource, map::world_xy_tile_xy},
};

#[derive(Component, Clone, Copy, Debug)]
pub struct Drink {
    pub per_second: f32,
}

pub fn drink(
    time: Res<Time>,
    mut thirsts: Query<(&Transform, &mut Thirst), Without<WaterSource>>,
    water_sources: Query<&TilePos, With<WaterSource>>,
    mut actions: Query<(&Actor, &mut ActionState, &Drink)>,
) {
    for (Actor(actor), mut action_state, drink) in actions.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let (actor_transform, mut actor_thirst) =
                    thirsts.get_mut(*actor).expect("Actor has no position and thirst.");
                let water_source_tile =
                    super::shared_drinking::find_closest_water_source(&water_sources, actor_transform);
                let actor_tile = world_xy_tile_xy(actor_transform.translation.xy());
                if crate::map_gen::map::is_neighbor(&actor_tile, &water_source_tile) {
                    actor_thirst.drink_progress += drink.per_second * time.delta_seconds();

                    if actor_thirst.drink_progress > actor_thirst.thirst {
                        actor_thirst.thirst = 0.0;
                        actor_thirst.drink_progress = 0.0;

                        *action_state = ActionState::Success;
                    }
                } else {
                    //We're too far away.
                    *action_state = ActionState::Failure;
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}
