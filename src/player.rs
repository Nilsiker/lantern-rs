use bevy::prelude::*;

use crate::{
    core::{MoveEntityEvent, Position},
    rendering::Char,
};

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, input_player);
        println!("PlayerPlugin loaded successfully")
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn((Player, Position::default(), Char('@')));
}

fn input_player(
    input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<MoveEntityEvent>,
    player: Query<Entity, With<Player>>,
) {
    let player_entity = player.get_single().unwrap();
    if input.just_pressed(KeyCode::KeyH) {
        events.send(MoveEntityEvent::new(player_entity, (-1, 0)));
    } else if input.just_pressed(KeyCode::KeyL) {
        events.send(MoveEntityEvent::new(player_entity, (1, 0)));
    } else if input.just_pressed(KeyCode::KeyJ) {
        events.send(MoveEntityEvent::new(player_entity, (0, -1)));
    } else if input.just_pressed(KeyCode::KeyK) {
        events.send(MoveEntityEvent::new(player_entity, (0, 1)));
    }
}
