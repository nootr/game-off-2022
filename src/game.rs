use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    Running,
    GameOver,
    WaitingForInput,
}

#[derive(Component)]
pub struct Game {
    pub state: GameState,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_game)
            .add_system(game_over_system);
    }
}

fn setup_game(mut commands: Commands) {
    commands.insert_resource(Game {
        state: GameState::Running,
    });
}

fn game_over_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
) {
    if game.state == GameState::GameOver {
        game.state = GameState::WaitingForInput;
        commands.spawn_bundle(TextBundle::from_section(
            "GAME\nOVER",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::RED,
            },
        ));
    }
}
