use bevy::{
    core::FrameCount,
    prelude::*,
    window::{PresentMode, WindowTheme},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use iyes_perf_ui::prelude::*;

const RES: (f32, f32) = (500.0, 500.0);
const MAX_ROUND: u8 = 5;
const PADDLE_WIDTH: f32 = 40.0;
const PADDLE_HEIGHT: f32 = 5.0;
const PADDLE_COLOR: Color = Color::srgb(0.0,0.7,0.0);

#[derive(Component)]
struct Player(u8);

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
            GraphicsPlugin,
        ))

        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        //.add_plugins(PerfUiPlugin)
        
        .add_systems(Startup, (startup_system, check_startup).chain())
        .run();
}

struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.0,0.0,0.0)));
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

fn startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>){
    commands.spawn(Camera2dBundle::default());
    commands.spawn(PerfUiCompleteBundle::default());
    
    
    let shape: Mesh2dHandle = Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_HEIGHT, PADDLE_WIDTH)));
    commands.insert_resource(GameState { current_round: 0, max_rounds: MAX_ROUND });
    commands.spawn((
        Player(1),
        Score(0),
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(PADDLE_COLOR),
            transform: Transform::from_xyz(- (RES.1/2.0 - 50.0),  0.0, 0.0),
            ..default()
        })
    );

    let shape: Mesh2dHandle = Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_HEIGHT, PADDLE_WIDTH)));
    commands.spawn((
        Player(2),
        Score(0),
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(PADDLE_COLOR),
            transform: Transform::from_xyz(RES.1 / 2.0 - 50.0, 0.0, 0.0),
            ..default()
        },
    ));
}

fn check_startup(query: Query<(&Player, &Transform, &Score)>, game_state: Res<GameState>){
    println!("-------------------------------------------------------------");
    println!("Game state => Current round : {} | Max rounds : {}", game_state.current_round, game_state.max_rounds);
    for (player, transform, score) in &query {
        println!("Player : {} | Position : {:?} | {}", player.0, transform, score.0);
    }
    println!("-------------------------------------------------------------");
}