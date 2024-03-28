mod core;
mod player;
mod rendering;

use core::{MoveEntityEvent, MovementPlugin};

use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;
use player::PlayerPlugin;
use rendering::CharRendererPlugin;

fn main() {
    App::new()
        .add_event::<MoveEntityEvent>()
        .add_plugins((
            DefaultPlugins,
            TerminalPlugin,
            PlayerPlugin,
            MovementPlugin,
            CharRendererPlugin,
        ))
        .run();
}
