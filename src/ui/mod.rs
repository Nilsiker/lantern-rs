use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

use crate::{
    player::Player,
    stats::{Health, Stat},
};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, draw_player_health_bar);
    }
}

fn draw_player_health_bar(
    mut terminal: Query<&mut Terminal>,
    player: Query<&Health, With<Player>>,
) {
    let mut terminal = terminal.single_mut();
    let health = player.single();

    terminal.put_string([0, 0].pivot(Pivot::TopLeft), "HP");

    let bar_start = [3, 0].pivot(Pivot::TopLeft);
    let bar_length = 10;
    let hp_text_pos = [bar_start.x() + bar_length + 1, 0].pivot(Pivot::TopLeft);
    let hp_text = format!("{}/{}", health.current(), health.max());

    terminal.put_string(
        [3, 0].pivot(Pivot::TopLeft),
        build_hp_string(health.current(), health.max(), bar_length as usize),
    );
    terminal.put_string(hp_text_pos, hp_text);
}

fn build_hp_string<'a>(current: usize, max: usize, bar_length: usize) -> FormattedString<'a> {
    let pips = (current as f32 / max as f32 * bar_length as f32).floor() as usize;
    let hp_string = format!("{}{}", "█".repeat(pips), " ".repeat(bar_length - pips))
        .fg(Color::RED)
        .bg(Color::DARK_GRAY);

    hp_string
}
