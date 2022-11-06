use bevy::prelude::*;
use bevy_ui_navigation::{
    prelude::*,
    systems::{default_mouse_input, update_boundaries, InputMapping},
    NavRequestSystem,
};

pub struct NavigationSystemsPlugin;

impl Plugin for NavigationSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputMapping>()
            .add_system(default_mouse_input.before(NavRequestSystem))
            .add_system(keyboard_input.before(NavRequestSystem))
            .add_system(update_boundaries.before(NavRequestSystem).after(default_mouse_input));
    }
}

pub fn keyboard_input(
    has_focused: Query<(), With<Focused>>,
    keyboard: Res<Input<KeyCode>>,
    mut nav_cmds: EventWriter<NavRequest>,
) {
    if has_focused.is_empty() {
        return;
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        dbg!("Cancel");
        nav_cmds.send(NavRequest::Cancel);
    }
}
