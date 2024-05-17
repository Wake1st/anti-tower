use bevy::prelude::*;
use bevy_rapier2d::{pipeline::CollisionEvent, plugin::RapierContext};

use crate::{bubble::Bubble, footman::Footman, health::Health, schedule::InGameSet};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_bubble_collisions.in_set(InGameSet::EntityUpdates),
            // apply_collision_damage::<Bubble, Footman>.in_set(InGameSet::EntityUpdates),
        );
    }
}

// #[derive(Component, Debug)]
// pub struct Collider {
//     pub radius: f32,
//     pub colliding_entities: Vec<Entity>,
// }

// impl Collider {
//     pub fn new(radius: f32) -> Self {
//         Self {
//             radius,
//             colliding_entities: vec![],
//         }
//     }
// }

#[derive(Component, Debug)]
pub struct CollisionDamage {
    pub amount: f32,
}

impl CollisionDamage {
    pub fn new(amount: f32) -> Self {
        Self { amount }
    }
}

// #[derive(Event, Debug)]
// pub struct CollisionEvent {
//     pub entity: Entity,
//     pub colliding_entity: Entity,
// }

// impl CollisionEvent {
//     pub fn new(entity: Entity, colliding_entity: Entity) -> Self {
//         Self {
//             entity,
//             colliding_entity,
//         }
//     }
// }

// fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
//     let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

//     //  phase 1: detect collisions
//     for (entity_a, transform_a, collider_a) in query.iter() {
//         for (entity_b, transform_b, collider_b) in query.iter() {
//             if entity_a != entity_b {
//                 let distance = transform_a
//                     .translation()
//                     .distance(transform_b.translation());

//                 //  here for weird transform::Zero bug
//                 if distance == 0.0 {
//                     continue;
//                 }

//                 if distance < (collider_a.radius + collider_b.radius) {
//                     colliding_entities
//                         .entry(entity_a)
//                         .or_insert_with(Vec::new)
//                         .push(entity_b);
//                 }
//             }
//         }
//     }

//     //  phase 2: update colliders
//     for (entity, _, mut collider) in query.iter_mut() {
//         collider.colliding_entities.clear();
//         if let Some(collisions) = colliding_entities.get(&entity) {
//             collider
//                 .colliding_entities
//                 .extend(collisions.iter().copied());
//         }
//     }
// }

// fn handle_collisions<T: Component>(
//     mut collision_event_writer: EventWriter<CollisionEvent>,
//     query: Query<(Entity, &Collider), With<T>>,
// ) {
//     for (entity, collider) in query.iter() {
//         for &colliding_entity in collider.colliding_entities.iter() {
//             //  entity colliding with another entity of the same type
//             if query.get(colliding_entity).is_ok() {
//                 continue;
//             }

//             //  send collision event
//             collision_event_writer.send(CollisionEvent::new(entity, colliding_entity));
//         }
//     }
// }

// pub fn apply_collision_damage<D: Component, H: Component>(
//     mut collision_event_reader: EventReader<CollisionEvent>,
//     mut health_query: Query<&mut Health, With<H>>,
//     damage_query: Query<(Entity, &CollisionDamage), With<D>>,
// ) {
//     for contact_event in collision_event_reader.read() {
//         info!("contact_event");
//         for (damage_entity, collision_damage) in damage_query.iter() {
//             if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
//                 info!("destructure event");

//                 info!("d_ent {:?}\t| h1 {:?}\t| h2 {:?}", damage_entity, h1, h2);

//                 if &damage_entity == h1 {
//                     let Ok(mut health) = health_query.get_mut(*h2) else {
//                         continue;
//                     };

//                     info!("health {:?} \t| damage {:?}", h2, h1);
//                     health.value -= collision_damage.amount;
//                 } else if &damage_entity == h2 {
//                     let Ok(mut health) = health_query.get_mut(*h1) else {
//                         continue;
//                     };

//                     info!("health {:?} \t| damage {:?}", h1, h2);
//                     health.value -= collision_damage.amount;
//                 } else {
//                     continue;
//                 }
//             }
//         }
//     }
// }

fn handle_bubble_collisions(
    rapier_context: Res<RapierContext>,
    query_bubble: Query<(Entity, &CollisionDamage), With<Bubble>>,
    mut query_footman: Query<(Entity, &mut Health), With<Footman>>,
    mut commands: Commands,
) {
    for (entity_bubble, collision_damage) in query_bubble.iter() {
        for (entity_footman, mut health) in query_footman.iter_mut() {
            if rapier_context.intersection_pair(entity_bubble, entity_footman) == Some(true) {
                commands.entity(entity_bubble).despawn();
                health.value -= collision_damage.amount;
            }
        }
    }
}
