use bevy::{
    prelude::*,
    utils::hashbrown::{hash_map::Entry, HashMap},
};

use crate::{
    bubble::{Bubble, BubbleSpawner},
    footman::Footman,
    group::Group,
    health::Health,
    movement::Velocity,
    schedule::InGameSet,
    tower::Tower,
};

const COLLISION_BUFFER: f32 = 2.0;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_detection.in_set(InGameSet::CollisionDetection),
        )
        .add_systems(
            Update,
            (
                (
                    handle_collisions::<Tower>,
                    handle_collisions::<Footman>,
                    handle_collisions::<BubbleSpawner>,
                    handle_collisions::<Bubble>,
                ),
                (
                    apply_collision_damage,
                    update_solid_collisions,
                    update_bouncy_collisions,
                ),
            )
                .chain()
                .in_set(InGameSet::EntityUpdates),
        )
        .insert_resource(CollisionRecords::new(HashMap::new()))
        .add_event::<CollisionEvent>();
    }
}

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

#[derive(Component, Debug)]
pub struct CollisionDamage {
    pub amount: f32,
}

impl CollisionDamage {
    pub fn new(amount: f32) -> Self {
        Self { amount }
    }
}

#[derive(Component, Debug)]
pub struct Bounce {
    pub value: f32,
}

impl Bounce {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Resource, Debug)]
pub struct CollisionRecords {
    pub value: HashMap<(Entity, Entity), bool>,
}

impl CollisionRecords {
    pub fn new(value: HashMap<(Entity, Entity), bool>) -> Self {
        Self { value }
    }
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub colliding_entity: Entity,
}

impl CollisionEvent {
    pub fn new(entity: Entity, colliding_entity: Entity) -> Self {
        Self {
            entity,
            colliding_entity,
        }
    }
}

/// Pairwise collision filtering using bit masks.
///
/// This filtering method is based on two 32-bit values:
/// - The interaction groups memberships.
/// - The interaction groups filter.
///
/// An interaction is allowed between two filters `a` and `b` when two conditions
/// are met simultaneously:
/// - The groups membership of `a` has at least one bit set to `1` in common with the groups filter of `b`.
/// - The groups membership of `b` has at least one bit set to `1` in common with the groups filter of `a`.
///
/// In other words, interactions are allowed between two filter iff. the following condition is met:
/// ```ignore
/// (self.memberships & rhs.filter) != 0 && (rhs.memberships & self.filter) != 0
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Component)]
// #[reflect(Component, Hash, PartialEq)]
pub struct CollisionGroups {
    /// Groups memberships.
    pub memberships: Group,
    /// Groups filter.
    pub filters: Group,
}

impl CollisionGroups {
    /// Creates a new collision-groups with the given membership masks and filter masks.
    pub const fn new(memberships: Group, filters: Group) -> Self {
        Self {
            memberships,
            filters,
        }
    }
}

fn collision_detection(
    mut query: Query<(Entity, &CollisionGroups, &GlobalTransform, &mut Collider)>,
    mut collision_records: ResMut<CollisionRecords>,
) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();
    // let mut filtered_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    //  phase 1: detect collisions
    for (entity_a, groups_a, transform_a, collider_a) in query.iter() {
        for (entity_b, groups_b, transform_b, collider_b) in query.iter() {
            //  cannot collide with self
            if entity_a == entity_b {
                continue;
            }

            //  first, check the groups for a match - [fastest check(?) should be first]
            if (groups_a.memberships & groups_b.filters) == Group::NONE {
                continue;
            }

            //  next, check if a collision would occur
            let distance = transform_a
                .translation()
                .distance(transform_b.translation());

            //  here for weird transform::Zero bug
            if distance == 0.0 {
                continue;
            }

            if distance < (collider_a.radius + collider_b.radius) {
                //  check for immediately previous collision
                match collision_records.value.entry((entity_a, entity_b)) {
                    Entry::Occupied(_) => {
                        let Some(_) = collision_records.value.remove(&(entity_a, entity_b)) else {
                            continue;
                        };
                        continue;
                    }
                    _ => {
                        collision_records.value.insert((entity_a, entity_b), true);
                    }
                }

                colliding_entities
                    .entry(entity_a)
                    .or_insert_with(Vec::new)
                    .push(entity_b);
            }
        }
    }

    //  phase 2: update colliders
    for (entity, _, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}

fn handle_collisions<T: Component>(
    mut collision_event_writer: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for &colliding_entity in collider.colliding_entities.iter() {
            //  entity colliding with another entity of the same type
            if query.get(colliding_entity).is_ok() {
                continue;
            }

            collision_event_writer.send(CollisionEvent::new(entity, colliding_entity));
        }
    }
}

pub fn apply_collision_damage(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut attacked_query: Query<&mut Health>,
    attacker_query: Query<&CollisionDamage, With<Bounce>>,
) {
    for &CollisionEvent {
        entity,
        colliding_entity,
    } in collision_event_reader.read()
    {
        let Ok(mut health) = attacked_query.get_mut(entity) else {
            continue;
        };

        let Ok(collision_damage) = attacker_query.get(colliding_entity) else {
            continue;
        };

        health.value -= collision_damage.amount;
    }
}

pub fn update_solid_collisions(
    mut collision_event_reader: EventReader<CollisionEvent>,
    attacked_query: Query<(&GlobalTransform, &Collider)>,
    mut attacker_query: Query<(&GlobalTransform, &Collider, &mut Transform), Without<Bounce>>,
) {
    for &CollisionEvent {
        entity,
        colliding_entity,
    } in collision_event_reader.read()
    {
        let Ok((attacked_transform, attacked_collider)) = attacked_query.get(entity) else {
            continue;
        };

        let Ok((attacker_global_transform, attacker_collider, mut attacker_transform)) =
            attacker_query.get_mut(colliding_entity)
        else {
            continue;
        };

        //  0: gather variables
        let planar_transform = Transform::from_xyz(
            attacked_transform.translation().x,
            attacked_transform.translation().y,
            attacker_global_transform.translation().z,
        );
        let deflection_vec =
            (attacker_global_transform.translation() - planar_transform.translation).normalize();
        let required_distance: f32 = attacked_collider.radius + attacker_collider.radius;
        let current_distance: f32 = attacker_global_transform
            .translation()
            .distance(planar_transform.translation);
        let adjusted_distance = required_distance - current_distance;

        //  1: "shift" the attacker off of the attacked to ensure no overlap
        attacker_transform.translation += deflection_vec * (adjusted_distance + COLLISION_BUFFER);
    }
}

pub fn update_bouncy_collisions(
    mut collision_event_reader: EventReader<CollisionEvent>,
    attacked_query: Query<(&GlobalTransform, &Collider)>,
    mut attacker_query: Query<
        (
            &GlobalTransform,
            &Collider,
            &mut Transform,
            &mut Velocity,
            &Bounce,
        ),
        With<Bounce>,
    >,
) {
    for &CollisionEvent {
        entity,
        colliding_entity,
    } in collision_event_reader.read()
    {
        let Ok((attacked_transform, attacked_collider)) = attacked_query.get(entity) else {
            continue;
        };

        let Ok((
            attacker_global_transform,
            attacker_collider,
            mut attacker_transform,
            mut attacker_velocity,
            bounce,
        )) = attacker_query.get_mut(colliding_entity)
        else {
            continue;
        };

        //  0: gather variables
        let planar_transform = Transform::from_xyz(
            attacked_transform.translation().x,
            attacked_transform.translation().y,
            attacker_global_transform.translation().z,
        );
        let deflection_vec =
            (attacker_global_transform.translation() - planar_transform.translation).normalize();
        let required_distance: f32 = attacked_collider.radius + attacker_collider.radius;
        let current_distance: f32 = attacker_global_transform
            .translation()
            .distance(planar_transform.translation);
        let adjusted_distance = required_distance - current_distance;

        //  1: "shift" the attacker off of the attacked to ensure no overlap
        attacker_transform.translation += deflection_vec * (adjusted_distance + COLLISION_BUFFER);

        //  2: "bounce" the attacker off the attacked
        let radial_velocity = deflection_vec.dot(attacker_velocity.value) * deflection_vec;
        attacker_velocity.value += -(1. + bounce.value) * radial_velocity;
    }
}
