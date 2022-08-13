pub mod setup;
pub mod assets;


pub mod prelude {
    pub use crate::{
        setup::*,
        assets::*
    };

    #[derive(Clone, Eq, PartialEq, Debug, Hash)]
    pub enum GameState {
        Running,
        Paused
    }
    
    #[derive(Clone, Eq, PartialEq, Debug, Hash)]
    pub enum AppState {
        Loading,
        Menu,
        InGame
    }
}