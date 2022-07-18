use bevy::{prelude::*, sprite::Material2dPlugin};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing,
}

#[derive(Component)]
struct PlayerCamera;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Fabien et le trahison de Olf".to_string(),
        // vsync: true,
        // mode: bevy::window::WindowMode::BorderlessFullscreen,
        ..WindowDescriptor::default()
    })
    .add_plugins(DefaultPlugins);

    app.run();
}

fn game_setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PlayerCamera);
}
