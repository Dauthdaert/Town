use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;
use big_brain::prelude::*;

use crate::{
    ai::behaviors::thirst::Thirst,
    map_gen::{components::WaterSource, map::tile_xy_world_xy},
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
                //Compute a path?
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let (actor_transform, mut actor_thirst) =
                    thirsts.get_mut(*actor).expect("Actor has no position and thirst.");
                let closest_water_source =
                    super::shared_drinking::find_closest_water_source(&water_sources, actor_transform);

                let distance = tile_xy_world_xy(closest_water_source.x, closest_water_source.y)
                    .distance(actor_transform.translation.xy());
                if distance <= super::MAX_ACTION_DISTANCE {
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
