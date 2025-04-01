use crate::GameState;
use bevy::prelude::*;
use crate::dev::prelude::*;
use clap::Parser;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadLevelEvent>();
        app.add_console_command::<LevelCommand, _>(level_command);
        app.add_systems(Update, load_level);
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "level")]
struct LevelCommand {
    id: String,
}

#[derive(Event)]
pub struct LoadLevelEvent(pub String);

fn level_command(
    mut ev_loadlevel: EventWriter<LoadLevelEvent>,
    mut log: ConsoleCommand<LevelCommand>,
) {
    if let Some(Ok(LevelCommand { id })) = log.take() {
        ev_loadlevel.send(LoadLevelEvent(id));
    }
}

fn load_level(
    mut commands: Commands,
    mut ev_loadlevel: EventReader<LoadLevelEvent>,
    // mut game_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_loadlevel.read() {
        commands.spawn(SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(format!("levels\\{}.glb", ev.0))
        )));
        // commands.spawn((
        //     Camera3d::default(),
        //     Transform::from_translation((0., 0., -4.).into()),
        // ));
    }
}