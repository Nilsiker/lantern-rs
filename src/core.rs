use bevy::prelude::*;

#[derive(Event)]
pub struct MoveEntityEvent {
    entity: Entity,
    direction: (isize, isize),
}
impl MoveEntityEvent {
    pub fn new(entity: Entity, direction: (isize, isize)) -> Self {
        Self { entity, direction }
    }
}

#[derive(Component, Default, Debug)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

fn move_entities(
    mut events: EventReader<MoveEntityEvent>,
    mut query: Query<(Entity, &mut Position)>,
) {
    for event in events.read() {
        let (_, mut position) = query.get_mut(event.entity).expect("entity exists");
        position.x += event.direction.0;
        position.y += event.direction.1;
    }
}

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveEntityEvent>()
            .add_systems(Update, move_entities);
    }
}
