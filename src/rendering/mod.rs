use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

use crate::core::Position;

#[derive(Component)]
pub struct Char(pub char);

fn add_terminal(mut commands: Commands, window: Query<&Window>) {
    let terminal = Terminal::new([20, 3]).with_border(Border::single_line());
    commands.spawn((TerminalBundle::from(terminal), AutoCamera));
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
        app.add_systems(Startup, add_terminal)
            .add_systems(Update, render_characters);
    }
}
