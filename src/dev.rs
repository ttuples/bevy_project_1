use crate::GameState;
use bevy::prelude::*;
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub(crate) mod prelude {
    pub use bevy_console::{reply, AddConsoleCommand, ConsoleCommand};
}

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::default());
    }
}