#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    prelude::*, 
    diagnostic::{FrameTimeDiagnosticsPlugin}
};
use game::prelude::{SetupPlugin, AssetsPlugin};

use bevy_ecs_ldtk::prelude::*;

//#[cfg(feature = "debug")]
//#[cfg(not(feature = "my_debug"))]

fn main() {
    let mut app: App = App::new();

    app
        .add_plugin(AssetsPlugin)
        .add_plugin(SetupPlugin)
        .add_plugins(DefaultPlugins);
    
    //map
    app
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Index(1))
         .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: false,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
    });

    if cfg!(debug_assertions) {
        app.add_plugin(FrameTimeDiagnosticsPlugin);
    }

    app.run();
}
