use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::map_gen::{
    components::{Choppable, Growing},
    map::Map,
    FeatureQuery,
};

pub fn grow(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut growing_query: Query<(Entity, &TilePos, &mut Growing)>,
    mut feature_query: FeatureQuery,
) {
    for (entity, tile_pos, mut growing) in growing_query.iter_mut() {
        growing.progress += growing.speed;

        if growing.progress >= 100.0 {
            let idx = map.tile_xy_idx(tile_pos.x, tile_pos.y);
            let current_feature = map.features[idx].expect("There should be a feature at growing location.");
            let next_feature = match current_feature {
                crate::map_gen::Features::TreeStump => crate::map_gen::Features::Tree,
                crate::map_gen::Features::CoconutTreeStump => crate::map_gen::Features::CoconutTree,
                _ => unreachable!(),
            };

            map.features[idx] = Some(next_feature);
            feature_query.change_feature_tile(entity, next_feature);

            if next_feature.is_choppable() {
                commands.entity(entity).insert(Choppable);
            }

            commands.entity(entity).remove::<Growing>();
        }
    }
}
