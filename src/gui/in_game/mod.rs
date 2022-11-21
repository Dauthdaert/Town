use bevy::prelude::*;
use bevy_ui_build_macros::*;
use bevy_ui_navigation::prelude::*;

use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    condition_set_in_states,
    gui::in_game::orders::InGameOrdersUiRoot,
    jobs::{JobCreationControls, JobCreationMenuManager},
    states::GameStates,
};

use super::UiAssets;

mod build;
mod orders;

#[derive(Component, Clone, Copy)]
struct InGameUiRoot;

#[allow(clippy::enum_variant_names)]
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum InGameUiMenu {
    OrdersMenu,
    BuildMenu,
}

impl InGameUiMenu {
    pub fn name(&self) -> &'static str {
        match self {
            InGameUiMenu::OrdersMenu => "OrdersMenuButton",
            InGameUiMenu::BuildMenu => "BuildMenuButton",
        }
    }

    pub fn to_button(self) -> (Button, Focusable, Interaction, Name) {
        (
            Button,
            Focusable::new(),
            Interaction::None,
            Name::new(self.name().to_string()),
        )
    }

    pub fn to_menu(self) -> (MenuSetting, MenuBuilder) {
        (MenuSetting::new(), MenuBuilder::from_named(self.name()))
    }
}

pub struct InGameGuiPlugin;

impl Plugin for InGameGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameStates::InGame, setup_in_game_ui)
            .add_system_set(
                condition_set_in_states!(GameStates::InGame | GameStates::InJobSelection)
                    .run_on_event::<NavEvent>()
                    .with_system(show_menus)
                    .with_system(orders::update_orders_menu_ui)
                    .with_system(build::update_build_menu_ui)
                    .into(),
            )
            // TODO!(2, Wayan, 8) : Don't hide menu when in job selection. Probably needs to hook into keyboard
            // navigation somehow.
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                condition_set_in_states!(GameStates::InGame | GameStates::InJobSelection)
                    .run_on_event::<NavEvent>()
                    .with_system(hide_menus)
                    .into(),
            );
    }
}

fn setup_in_game_ui(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    mut in_game_ui_query: Query<&mut Visibility, With<InGameUiRoot>>,
) {
    if let Ok(mut root) = in_game_ui_query.get_single_mut() {
        root.is_visible = true;
    } else {
        let text_bundle = |content: &str, font_size: f32| ui_assets.text_bundle(content, font_size);
        let node = NodeBundle {
            background_color: Color::NONE.into(),
            style: style! {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
            },
            ..default()
        };
        let button = NodeBundle {
            style: style! {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: rect!(20 px),
                margin: rect!(5 px),
                size: size!(110 px, 60 px),
            },
            ..default()
        };

        build_ui! {
            #[cmd(commands)]
            node {
                min_size: size!(100 pct, 100 pct),
                justify_content: JustifyContent::Center
            }[; Name::new("InGameRootNode"), InGameUiRoot](
                node {
                    position_type: PositionType::Absolute,
                    position: rect!(15 px, auto, auto, 15 px,),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::FlexStart
                }[; MenuSetting::new(), MenuBuilder::Root](
                    node {
                        flex_direction: FlexDirection::Column
                    }[;](
                        button {}[InGameUiMenu::OrdersMenu.to_button(); Focusable::new().prioritized()](
                            node[text_bundle("Orders", 20.0);]
                        ),
                        node {
                            flex_direction: FlexDirection::Column
                        }[InGameUiMenu::OrdersMenu.to_menu(); InGameOrdersUiRoot](
                            button {
                                display: Display::None
                            }[orders::InGameOrdersUiElem::ChopButton.to_button();](
                                node[text_bundle("Chop", 20.0);]
                            ),
                            button {
                                display: Display::None
                            }[orders::InGameOrdersUiElem::MineButton.to_button();](
                                node[text_bundle("Mine", 20.0);]
                            ),
                            button {
                                display: Display::None
                            }[orders::InGameOrdersUiElem::ClearButton.to_button();](
                                node[text_bundle("Clear", 20.0);]
                            )
                        )
                    ),
                    node{
                        flex_direction: FlexDirection::Column
                    }[;](
                        button {}[InGameUiMenu::BuildMenu.to_button();](
                            node[text_bundle("Build", 20.0);]
                        ),
                        node {
                            flex_direction: FlexDirection::Column
                        }[InGameUiMenu::BuildMenu.to_menu(); build::InGameBuildUiRoot](
                            button {
                                display: Display::None
                            }[build::InGameBuildUiElem::BuildRoadButton.to_button();](
                                node[text_bundle("Road", 20.0);]
                            ),
                            button {
                                display: Display::None
                            }[build::InGameBuildUiElem::BuildRoomButton.to_button();](
                                node[text_bundle("Room", 20.0);]
                            )
                        )
                    )
                )
            )
        };
    }
}

fn show_menus(
    jobs_menu: Query<&InGameUiMenu, Added<Focused>>,
    mut elements: Query<(&mut Style, &InGameUiMenu)>,
    mut job_creation_controls: Query<&mut ActionState<JobCreationControls>, With<JobCreationMenuManager>>,
) {
    if let Ok(menu) = jobs_menu.get_single() {
        job_creation_controls.single_mut().press(JobCreationControls::Exit);
        for (mut style, element_type) in elements.iter_mut() {
            if *menu == *element_type {
                style.display = Display::Flex;
            }
        }
    }
}

fn hide_menus(
    jobs_menu: Query<&InGameUiMenu>,
    mut elements: Query<(&mut Style, &InGameUiMenu)>,
    removals: RemovedComponents<Focused>,
) {
    for entity in removals.iter() {
        if let Ok(menu) = jobs_menu.get(entity) {
            for (mut style, element_type) in elements.iter_mut() {
                if *menu == *element_type {
                    style.display = Display::None;
                }
            }
        }
    }
}
