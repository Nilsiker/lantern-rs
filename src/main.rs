mod core;
mod player;
mod rendering;
mod stats;
mod ui;

use core::{MoveEntityEvent, MovementPlugin};

use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;
use player::PlayerPlugin;
use rendering::CharRendererPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_event::<MoveEntityEvent>()
        .add_plugins((
            DefaultPlugins,
            TerminalPlugin,
            PlayerPlugin,
            MovementPlugin,
            CharRendererPlugin,
            UiPlugin,
        ))
        .run();
}
