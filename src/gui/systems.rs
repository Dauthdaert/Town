use bevy::prelude::*;
use bevy_ui_navigation::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

#[allow(clippy::type_complexity)]
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Focusable>, With<Button>, Changed<Interaction>),
    >,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        *material = match *interaction {
            Interaction::Clicked => HOVERED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        };
    }
}
