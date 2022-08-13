use bevy::{prelude::*, render::texture::ImageSettings};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
            .add_startup_system(startup_images)
            .add_startup_system(startup_fonts)
            .add_startup_system(startup_sounds);
    }
}

pub struct Fonts;

pub struct Images;

pub struct Sounds;

fn startup_images(
    mut _commands: Commands,
    _asset_server: Res<AssetServer>,
) {

}

fn startup_fonts(
    mut _commands: Commands,
    _asset_server: Res<AssetServer>,
) {

}

fn startup_sounds(
    mut _commands: Commands,
    _asset_server: Res<AssetServer>,
) {

}