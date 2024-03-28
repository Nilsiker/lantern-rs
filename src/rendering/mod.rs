use bevy::prelude::*;
use bevy_ascii_terminal::{prelude::*, TiledCameraBundle};

use crate::core::Position;

#[derive(Component)]
pub struct Char(pub char);

fn add_terminal(mut commands: Commands) {
    let terminal = Terminal::new([80, 45]);
    let terminal_size = terminal.size_with_border();
    println!("{terminal_size:?}");
    commands.spawn(TerminalBundle::from(terminal));
    commands.spawn(TiledCameraBundle::unit_cam([82, 47]));
}

fn render_characters(query: Query<(&Char, &Position)>, mut terminal: Query<&mut Terminal>) {
    let Ok(mut terminal) = terminal.get_single_mut() else {
        return;
    };

    terminal.clear();
    for (character, position) in &query {
        let pos = [position.x as usize, position.y as usize];
        if terminal.in_bounds(pos) {
            terminal.put_char(pos, character.0);
        }
    }
}

pub struct CharRendererPlugin;
impl Plugin for CharRendererPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_systems(Startup, add_terminal)
            .add_systems(Update, render_characters);
    }
}
