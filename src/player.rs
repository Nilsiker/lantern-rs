use bevy::{
    ecs::event::EventId,
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use bevy_ascii_terminal::{GridPoint, Pivot, StringFormatter, Terminal};

use crate::{
    core::{MoveEntityEvent, Position},
    rendering::Char,
    stats::{Health, Stat},
};

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(PostUpdate, input_event_player); // TODO fix rendering pipeline for UI
        println!("PlayerPlugin loaded successfully")
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Position::default(),
        Char('@'),
        Health::new_at(70, 100),
    ));
}

fn input_event_player(
    mut events: EventReader<KeyboardInput>,
    mut move_events: EventWriter<MoveEntityEvent>,
    player: Query<Entity, With<Player>>,
) {
    let player_entity = player.get_single().unwrap();
    for event in events.read().filter(|ev| ev.state == ButtonState::Pressed) {
        match event.key_code {
            KeyCode::KeyH => move_events.send(MoveEntityEvent::new(player_entity, (-1, 0))),
            KeyCode::KeyJ => move_events.send(MoveEntityEvent::new(player_entity, (0, -1))),
            KeyCode::KeyL => move_events.send(MoveEntityEvent::new(player_entity, (1, 0))),
            KeyCode::KeyK => move_events.send(MoveEntityEvent::new(player_entity, (0, 1))),
            _ => move_events.send(MoveEntityEvent::new(player_entity, (0, 0))),
        };
        if event.key_code == KeyCode::KeyH && event.state == ButtonState::Pressed {}
    }
}
