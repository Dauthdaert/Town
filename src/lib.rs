#![allow(clippy::too_many_arguments)]

use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode, winit::WinitSettings};
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_mouse_tracking_plugin::prelude::*;
use bevy_turborand::RngPlugin;
use iyes_loopless::prelude::*;
use iyes_progress::ProgressPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
#[cfg(debug_assertions)]
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

pub const LAUNCHER_TITLE: &str = "Town";

pub const SIMULATION_SPEED: f32 = 5.0;

mod ai;
mod animation;
mod camera;
pub mod jobs;
mod map;
mod simulation;
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
    .insert_resource(WinitSettings::game())
    .insert_resource(ImageSettings::default_nearest())
    .insert_resource(Msaa { samples: 1 });

    //Add bevy and dependency plugins
    app.add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(MousePosPlugin)
        .add_plugin(RngPlugin::default());

    #[cfg(debug_assertions)]
    {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default())
            .insert_resource(WorldInspectorParams {
                enabled: true,
                ..Default::default()
            })
            .add_plugin(WorldInspectorPlugin::new());
    }

    //Add custom resources and systems
    app.add_loopless_state(GameStates::Splash)
        .add_loading_state(
            LoadingState::new(GameStates::Splash)
                .continue_to_state(GameStates::MapGeneration)
                .with_collection::<map::TilemapAssets>()
                .with_collection::<animation::SpriteAssets>()
                .with_collection::<jobs::JobCreationMenuAssets>(),
        )
        .add_plugin(ProgressPlugin::new(GameStates::Splash));

    app.add_plugin(ProgressPlugin::new(GameStates::InGamePrepare).continue_to(GameStates::InGame));

    //Add custom plugins
    app.add_plugin(camera::CameraPlugin)
        .add_plugin(map::MapGenPlugin)
        .add_plugin(jobs::JobsPlugin)
        .add_plugin(ai::AIPlugin)
        .add_plugin(simulation::SimulationPlugin)
        .add_plugin(animation::AnimationPlugin);

    app
}

fn cleanup_resource<T: bevy::ecs::system::Resource>(mut commands: Commands) {
    commands.remove_resource::<T>();
}

fn cleanup_entity_by_component<T: bevy::ecs::component::Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[macro_export]
macro_rules! condition_set_in_states {
    ($(|)? $( $pattern:pat_param )|+) => {
        iyes_loopless::prelude::ConditionSet::new()
            .run_if(move |res: Option<bevy::prelude::Res<iyes_loopless::prelude::CurrentState<$crate::states::GameStates>>>| {
                if_chain::if_chain! {
                    if let Some(res) = res;
                    let res = res.0;
                    then {
                        matches!(res, $( $pattern )|+)
                    } else {
                        false
                    }
                }
            })
    }
}
