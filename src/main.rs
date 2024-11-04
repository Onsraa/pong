use bevy::{
    core::FrameCount,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme},
};

const RES: (f32, f32) = (500.0, 500.0);

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
        ))
        .add_systems(Update, (make_visible, toggle_vsync))
        .run();
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