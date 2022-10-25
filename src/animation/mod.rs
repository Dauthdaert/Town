use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;

use crate::states::GameStates;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(AssetCollection)]
pub struct SpriteAssets {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 1))]
    #[asset(path = "textures/dwarf_villager_idle.png")]
    pub villager: Handle<TextureAtlas>,
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_sprite.run_in_state(GameStates::InGame));
    }
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut sprite_query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut sprite_query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
