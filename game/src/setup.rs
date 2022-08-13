use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

const GAME_ICON_PATH: &str = "game_icon.png";

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(camera_setup)
            .add_startup_system(set_window_icon)
            .add_startup_system(map_setup);

        app.insert_resource(WindowDescriptor {
            width: 600.,
            height: 600.,
            title: "game".to_string(),
            resizable: true,
            ..Default::default()
        });
    }
}

fn camera_setup(
    mut commands: Commands
) {
    let mut camera = Camera2dBundle::default();

    camera.transform.translation = Vec3::new(119., 199., 0.);

    commands.spawn_bundle(camera);
}

fn map_setup(mut commands: Commands, asset_server: Res<AssetServer>) {   
    
    let ldtk_handle = asset_server.load("maps/demo_map.ldtk");
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
    
}
/*
    A hacky way to set window icon from: https://bevy-cheatbook.github.io/window/icon.html
    winit = "0.26.1"
    image = "0.23.14" (https://crates.io/crates/image)
 */
use bevy::winit::WinitWindows;

fn set_window_icon(windows: NonSend<WinitWindows>) {
    use bevy::window::WindowId;
    use winit::window::Icon;

    let primary = windows.get_window(WindowId::primary()).unwrap();

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(GAME_ICON_PATH)
            .expect("Failed to open game_icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
}
