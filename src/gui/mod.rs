use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ui_navigation::prelude::*;

mod in_game;
mod systems;

#[derive(AssetCollection)]
pub struct UiAssets {
    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    pub font_medium: Handle<Font>,
}

impl UiAssets {
    pub fn text_bundle(&self, content: &str, font_size: f32) -> TextBundle {
        let color = Color::ANTIQUE_WHITE;
        let style = TextStyle {
            color,
            font: self.font_medium.clone(),
            font_size,
        };
        TextBundle::from_section(content, style)
    }
}

#[allow(dead_code)]
fn hide_ui_by_root_component<T: Component>(mut root_query: Query<&mut Visibility, With<T>>) {
    root_query.single_mut().is_visible = false;
}

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultNavigationPlugins)
            .add_system(systems::button_system.after(NavRequestSystem));

        app.add_plugin(in_game::InGameGuiPlugin);
    }
}
