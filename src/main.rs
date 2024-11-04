use bevy::{
    core::FrameCount,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme},
};

const RES: (f32, f32) = (500.0, 500.0);

#[derive(Component)]
struct Player(u8);

#[derive(Debug)]
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Component)]
struct Score(u32);

#[derive(Resource)]
struct GameState {
    current_round: u8,
    max_rounds: u8,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pong game".into(),
                    resolution:(RES.0, RES.1).into(),
                    present_mode: PresentMode::AutoNoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    visible: false,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            GraphicsPlugin,
        ))
        .add_systems(Startup, (startup_system, show_players).chain())
        .run();
}

struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (make_visible, toggle_vsync));
    }
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.single_mut().visible = true;
    }
}

fn toggle_vsync(input: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>){
    if input.just_pressed(KeyCode::KeyV){
        let mut window = windows.single_mut();

        window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync) {
            PresentMode::AutoNoVsync
        }else{
            PresentMode::AutoVsync
        };
        info!("PRESENT MODE : {:?}", window.present_mode)
    }
}

fn startup_system(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
    commands.spawn((Player(1), Position {x: RES.0 / 2.0, y: 100.0}, Score(0)));
    commands.spawn((Player(2), Position {x: RES.0 / 2.0, y: RES.1 - 100.0}, Score(0)));
}

fn show_players(query: Query<(&Player, &Position, &Score)>){
    for (player, position, score) in &query {
        println!("Player : {} | Position : {:?} | {}", player.0, position, score.0);
    }
}