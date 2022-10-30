use bevy::prelude::*;
use bevy_ui_build_macros::*;
use iyes_loopless::prelude::*;

use crate::{cleanup_entity_by_component, states::GameStates};

use super::UiAssets;

#[derive(Component, Clone, Copy)]
struct SplashUiRoot;

pub struct SplashGuiPlugin;

impl Plugin for SplashGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            setup_splash_ui
                .run_in_state(GameStates::Splash)
                .run_if_resource_exists::<UiAssets>(),
        )
        .add_exit_system(GameStates::InGamePrepare, cleanup_entity_by_component::<SplashUiRoot>);
    }
}

fn setup_splash_ui(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    splash_ui_query: Query<Entity, With<SplashUiRoot>>,
) {
    if splash_ui_query.is_empty() {
        let image = ImageBundle {
            image: ui_assets.splash_bg.clone().into(),
            style: style! {
                position_type: PositionType::Absolute,
                size: size!(auto, 100 pct),
            },
            ..default()
        };
        let node = NodeBundle {
            color: Color::NONE.into(),
            style: style! {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
            },
            ..default()
        };

        build_ui! {
            #[cmd(commands)]
            node {
                min_size: size!(100 pct, 100 pct),
                justify_content: JustifyContent::Center
            }[; Name::new("SplashRootNode"), SplashUiRoot](
                entity[image;]
            )
        };
    }
}
