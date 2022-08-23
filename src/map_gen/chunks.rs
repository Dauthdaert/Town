use bevy::{
    math::Vec3Swizzles,
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_ecs_tilemap::prelude::*;

use super::{generator::MapGenerator, TilemapAssets, TILE_SIZE};

const CHUNK_SIZE: TilemapSize = TilemapSize { x: 6, y: 6 };

#[derive(Default, Debug)]
pub struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
    pub chunks: HashMap<IVec2, Vec<u8>>,
}

impl ChunkManager {
    pub fn get_tile_texture(&mut self, chunk_pos: IVec2, tile_x: u32, tile_y: u32) -> Option<u8> {
        let tiles = self
            .chunks
            .entry(chunk_pos)
            .or_insert(vec![255; (CHUNK_SIZE.x * CHUNK_SIZE.y) as usize]);
        if tiles[chunk_xy_to_idx(tile_x, tile_y)] == 255 {
            None
        } else {
            Some(tiles[chunk_xy_to_idx(tile_x, tile_y)])
        }
    }

    pub fn set_tile_texture(&mut self, chunk_pos: IVec2, tile_x: u32, tile_y: u32, tile_texture: u8) {
        let tiles = self
            .chunks
            .entry(chunk_pos)
            .or_insert(vec![255; (CHUNK_SIZE.x * CHUNK_SIZE.y) as usize]);
        tiles[chunk_xy_to_idx(tile_x, tile_y)] = tile_texture;
    }
}

pub fn spawn_chunks_around_camera(
    mut commands: Commands,
    tilemap_assets: Res<TilemapAssets>,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    mut chunk_manager: ResMut<ChunkManager>,
    map_generator: Res<MapGenerator>,
) {
    let (camera_transform, camera_ortho) = camera_query.single();
    let chunk_spawn_distance: i32 = ((camera_ortho.right - camera_ortho.left) * 0.5 * camera_ortho.scale
        / (CHUNK_SIZE.x as f32 * TILE_SIZE.x))
        .ceil() as i32
        + 1;

    let camera_chunk_pos = camera_pos_to_chunk_pos(&camera_transform.translation.xy());
    for y in (camera_chunk_pos.y - chunk_spawn_distance)..(camera_chunk_pos.y + chunk_spawn_distance) {
        for x in (camera_chunk_pos.x - chunk_spawn_distance)..(camera_chunk_pos.x + chunk_spawn_distance) {
            if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                spawn_chunk(
                    &mut commands,
                    &tilemap_assets,
                    &mut chunk_manager,
                    IVec2::new(x, y),
                    &map_generator,
                );
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
    let chunk_remove_distance: f32 = (camera_ortho.right - camera_ortho.left) * camera_ortho.scale;

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

fn spawn_chunk(
    commands: &mut Commands,
    tilemap_assets: &TilemapAssets,
    chunk_manager: &mut ChunkManager,
    chunk_pos: IVec2,
    map_generator: &MapGenerator,
) {
    let tilemap_entity = commands.spawn().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE);
    // Spawn the elements of the tilemap.
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let mut tile_texture = chunk_manager.get_tile_texture(chunk_pos, x, y);

            //If no tile exists, generate one
            if tile_texture.is_none() {
                let result = map_generator
                    .generate(
                        chunk_pos.x * CHUNK_SIZE.x as i32 + x as i32,
                        chunk_pos.y * CHUNK_SIZE.y as i32 + y as i32,
                    )
                    .texture();
                chunk_manager.set_tile_texture(chunk_pos, x, y, result);
                tile_texture = Some(result);
            }

            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture: TileTexture(tile_texture.unwrap() as u32),
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, Some(tile_entity));
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

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}

fn chunk_xy_to_idx(x: u32, y: u32) -> usize {
    (y * CHUNK_SIZE.x + x) as usize
}
