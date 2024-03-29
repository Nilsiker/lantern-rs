use bevy::prelude::*;
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin))
        .add_systems(Startup, spawn)
        .add_systems(Update, input)
        .run();
}

const INFO_STRING: &str = "Arrow keys to resize\nSpacebar to change border";

fn spawn(mut commands: Commands) {
    let mut term = Terminal::new([25, 2]).with_border(Border::single_line());
    term.put_string([0, 0].pivot(Pivot::TopLeft), INFO_STRING);
    commands.spawn((TerminalBundle::from(term), AutoCamera));
}

fn input(
    input: Res<ButtonInput<KeyCode>>,
    mut q_term: Query<&mut Terminal>,
    mut index: Local<usize>,
) {
    let borders = &[
        Some(Border::single_line()),
        Some(Border::double_line()),
        None,
    ];

    let mut cleared = false;
    if input.just_pressed(KeyCode::Space) {
        let mut term = q_term.single_mut();

        *index = (*index + 1) % borders.len();
        if let Some(border) = borders[*index].clone() {
            term.set_border(border);
        } else {
            term.remove_border();
        }
        cleared = true;
    }

    if input.just_pressed(KeyCode::ArrowRight) {
        let mut term = q_term.single_mut();

        let size = [term.width() + 1, term.height()];
        term.resize(size);
        cleared = true;
    }
    if input.just_pressed(KeyCode::ArrowLeft) {
        let mut term = q_term.single_mut();

        let size = [term.width().saturating_sub(1).max(1), term.height()];
        term.resize(size);
        cleared = true;
    }
    if input.just_pressed(KeyCode::ArrowUp) {
        let mut term = q_term.single_mut();

        let size = [term.width(), term.height() + 1];
        term.resize(size);
        cleared = true;
    }
    if input.just_pressed(KeyCode::ArrowDown) {
        let mut term = q_term.single_mut();

        let size = [term.width(), term.height().saturating_sub(1).max(1)];
        term.resize(size);
        cleared = true;
    }

    if cleared {
        let mut term = q_term.single_mut();
        term.put_string([0, 0].pivot(Pivot::TopLeft), INFO_STRING);
    }

    let term = q_term.single_mut();
    println!("{:?}", term.size())
}
