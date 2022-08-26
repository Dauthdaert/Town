use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode};
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_turborand::RngPlugin;
use iyes_loopless::prelude::*;
use iyes_progress::ProgressPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub const LAUNCHER_TITLE: &str = "Town";

mod ai;
mod animation;
mod camera;
mod map_gen;
pub mod states;

pub fn app() -> App {
    use states::GameStates;

    let mut app = App::new();

    //Add bevy resources
    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        present_mode: PresentMode::AutoVsync,
        ..Default::default()
    })
    .insert_resource(ImageSettings::default_nearest())
    .insert_resource(Msaa { samples: 1 });

    //Add bevy and dependency plugins
    app.add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(RngPlugin::default());

    #[cfg(debug_assertions)]
    {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default());
    }

    //Add custom resources and systems
    app.add_loopless_state(GameStates::AssetLoading)
        .add_loading_state(
            LoadingState::new(GameStates::AssetLoading)
                .continue_to_state(GameStates::MapGeneration)
                .with_collection::<map_gen::TilemapAssets>()
                .with_collection::<animation::SpriteAssets>(),
        )
        .add_plugin(ProgressPlugin::new(GameStates::AssetLoading));

    //Add custom plugins
    app.add_plugin(camera::CameraPlugin)
        .add_plugin(map_gen::MapGenPlugin)
        .add_plugin(ai::AIPlugin)
        .add_plugin(animation::AnimationPlugin);

    app
}
