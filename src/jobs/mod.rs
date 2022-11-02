use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

mod cursor;
mod job_creation;
pub mod job_queue;

pub use job_creation::SelectionStart;
use job_queue::*;

use crate::{cleanup_entity_by_component, cleanup_resource, map::Features, states::GameStates};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Jobs {
    Chop,
    Build(Features),
}

impl Jobs {
    pub fn speed(&self) -> f32 {
        match self {
            Jobs::Chop => 20.0,
            Jobs::Build(_) => 30.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JobCreation {
    Chop,
    Build(Features),
    BuildRoom,
}

#[derive(Clone, Copy, Debug, Deref)]
pub struct JobSelectionType(pub JobCreation);

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq)]
pub enum JobCreationControls {
    Select,
    Exit,
}

#[derive(AssetCollection)]
pub struct JobCreationMenuAssets {
    #[asset(path = "textures/cursor.png")]
    pub cursor: Handle<Image>,
}

pub struct JobsPlugin;

impl Plugin for JobsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<JobQueue>()
            .add_plugin(InputManagerPlugin::<JobCreationControls>::default())
            .add_enter_system(GameStates::InGame, setup_job_manager);

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameStates::InJobSelection)
                .with_system(job_creation::handle_job_creation_hotkeys)
                .with_system(cursor::job_creation_menu_cursor_follow_mouse)
                .with_system(handle_job_exit_hotkeys)
                .into(),
        )
        .add_enter_system(GameStates::InJobSelection, cursor::setup_job_creation_menu_cursor)
        .add_exit_system(GameStates::InJobSelection, cleanup_resource::<JobSelectionType>)
        .add_exit_system(
            GameStates::InJobSelection,
            cleanup_entity_by_component::<cursor::JobCreationMenuCursor>,
        );
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct JobCreationMenuManager;

fn setup_job_manager(mut commands: Commands, manager_query: Query<Entity, With<JobCreationMenuManager>>) {
    if manager_query.is_empty() {
        commands
            .spawn_bundle(InputManagerBundle::<JobCreationControls> {
                action_state: ActionState::default(),
                input_map: InputMap::default()
                    .insert(KeyCode::Return, JobCreationControls::Select)
                    .insert(MouseButton::Left, JobCreationControls::Select)
                    .insert(KeyCode::Escape, JobCreationControls::Exit)
                    .build(),
            })
            .insert_bundle((JobCreationMenuManager, Name::from("Job Creation Menu Manager")));
    }
}

fn handle_job_exit_hotkeys(
    mut commands: Commands,
    selection: Option<Res<job_creation::SelectionStart>>,
    query: Query<&ActionState<JobCreationControls>, With<JobCreationMenuManager>>,
) {
    let job_creation_menu = query.single();

    if job_creation_menu.just_pressed(JobCreationControls::Exit) {
        if selection.is_some() {
            commands.remove_resource::<job_creation::SelectionStart>();
        } else {
            commands.insert_resource(NextState(GameStates::InGame));
        }
    }
}
