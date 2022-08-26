use bevy::{math::Vec3Swizzles, prelude::*, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;

use super::{map::Map, TilemapAssets, TILE_SIZE};

const CHUNK_SIZE: TilemapSize = TilemapSize { x: 16, y: 16 };

#[derive(Default, Debug)]
pub struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

pub fn spawn_chunks_around_camera(
    mut commands: Commands,
    tilemap_assets: Res<TilemapAssets>,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    mut chunk_manager: ResMut<ChunkManager>,
    map: Res<Map>,
) {
    let (camera_transform, camera_ortho) = camera_query.single();
    let chunk_spawn_distance: i32 = ((camera_ortho.right - camera_ortho.left) * 0.5 * camera_ortho.scale
        / (CHUNK_SIZE.x as f32 * TILE_SIZE.x))
        .ceil() as i32
        + 1;

    let camera_chunk_pos = camera_pos_to_chunk_pos(&camera_transform.translation.xy());
    for y in (camera_chunk_pos.y - chunk_spawn_distance)..(camera_chunk_pos.y + chunk_spawn_distance) {
        for x in (camera_chunk_pos.x - chunk_spawn_distance)..(camera_chunk_pos.x + chunk_spawn_distance) {
            if y >= 0
                && y as u32 * CHUNK_SIZE.y < map.height
                && x >= 0
                && x as u32 * CHUNK_SIZE.x < map.width
                && !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y))
            {
                chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                spawn_chunk(&mut commands, &tilemap_assets, IVec2::new(x, y), &map);
            }
        }
    }
}

pub fn despawn_chunks_outside_camera(
    mut commands: Commands,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    chunks_query: Query<(Entity, &Transform), With<TileStorage>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let (camera_transform, camera_ortho) = camera_query.single();
    let chunk_remove_distance: f32 = (camera_ortho.right - camera_ortho.left) * camera_ortho.scale * 2.;

    for (entity, chunk_transform) in chunks_query.iter() {
        let chunk_pos = chunk_transform.translation.xy();
        let distance = camera_transform.translation.xy().distance(chunk_pos);
        if distance > chunk_remove_distance {
            let x = (chunk_pos.x as f32 / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
            let y = (chunk_pos.y as f32 / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
            chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_chunk(commands: &mut Commands, tilemap_assets: &TilemapAssets, chunk_pos: IVec2, map: &Map) {
    if chunk_pos.x >= 0 && chunk_pos.y >= 0 {
        let tilemap_entity = commands.spawn().id();
        let mut tile_storage = TileStorage::empty(CHUNK_SIZE);
        // Spawn the elements of the tilemap.
        for x in 0..CHUNK_SIZE.x {
            for y in 0..CHUNK_SIZE.y {
                let abs_tile_x = chunk_pos.x as u32 * CHUNK_SIZE.x + x;
                let abs_tile_y = chunk_pos.y as u32 * CHUNK_SIZE.y + y;
                if abs_tile_y < map.height && abs_tile_x < map.width {
                    let tile_biome = map.tiles[map.xy_idx(abs_tile_x, abs_tile_y)];
                    let tile_pos = TilePos { x, y };

                    let tile_entity = commands
                        .spawn()
                        .insert_bundle(TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            texture: TileTexture(tile_biome.texture()),
                            ..Default::default()
                        })
                        .id();
                    commands.entity(tilemap_entity).add_child(tile_entity);
                    tile_storage.set(&tile_pos, Some(tile_entity));
                }
            }
        }

        let chunk_transform = Transform::from_translation(Vec3::new(
            chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
            chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
            0.0,
        ));
        commands.entity(tilemap_entity).insert_bundle(TilemapBundle {
            grid_size: TILE_SIZE.into(),
            size: CHUNK_SIZE,
            storage: tile_storage,
            texture: TilemapTexture(tilemap_assets.tiles.clone()),
            tile_size: TILE_SIZE,
            transform: chunk_transform,
            ..Default::default()
        });
    }
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}
