use bevy::{
    prelude::*,
    utils::hashbrown::{hash_map::Entry, HashMap},
};

use crate::{
    bubble::Bubble, footman::Footman, health::Health, movement::Velocity, schedule::InGameSet,
};

const BOUNCE_BUFFER: f32 = 2.0;

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
                (handle_collisions::<Footman>, handle_collisions::<Bubble>),
                (apply_collision_damage, update_collision_transforms),
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

// #[derive(Component, Debug)]
// pub struct CollisionRecord {
//     pub entity: Entity,
//     pub colliding_entity: Entity,
// }

// impl CollisionRecord {
//     pub fn new(entity: Entity, colliding_entity: Entity) -> Self {
//         Self {
//             entity,
//             colliding_entity,
//         }
//     }
// }

// #[derive(Resource, Debug)]
// pub struct CollisionRecords {
//     pub value: HashMap<Entity, Vec<Entity>>,
// }

// impl CollisionRecords {
//     pub fn new(value: HashMap<Entity, Vec<Entity>>) -> Self {
//         Self { value }
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

//  borrowed from rapier
/// A bit mask identifying groups for interaction.
#[derive(Component, Reflect, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[reflect(Component, Hash, PartialEq)]
// #[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Group(u32);

bitflags::bitflags! {
    impl Group: u32 {
        /// The group n°1.
        const PLAYER = 1 << 0;
        /// The group n°2.
        const ALLY = 1 << 1;
        /// The group n°3.
        const NPC = 1 << 2;
        /// The group n°4.
        const ENEMY = 1 << 3;
        /// The group n°5.
        const WEAPON = 1 << 4;
        /// The group n°6.
        const PROJECTILE = 1 << 5;
        /// The group n°7.
        const STRUCTURE = 1 << 6;
        /// The group n°8.
        const GROUP_8 = 1 << 7;
        /// The group n°9.
        const GROUP_9 = 1 << 8;
        /// The group n°10.
        const GROUP_10 = 1 << 9;
        /// The group n°11.
        const GROUP_11 = 1 << 10;
        /// The group n°12.
        const GROUP_12 = 1 << 11;
        /// The group n°13.
        const GROUP_13 = 1 << 12;
        /// The group n°14.
        const GROUP_14 = 1 << 13;
        /// The group n°15.
        const GROUP_15 = 1 << 14;
        /// The group n°16.
        const GROUP_16 = 1 << 15;
        /// The group n°17.
        const GROUP_17 = 1 << 16;
        /// The group n°18.
        const GROUP_18 = 1 << 17;
        /// The group n°19.
        const GROUP_19 = 1 << 18;
        /// The group n°20.
        const GROUP_20 = 1 << 19;
        /// The group n°21.
        const GROUP_21 = 1 << 20;
        /// The group n°22.
        const GROUP_22 = 1 << 21;
        /// The group n°23.
        const GROUP_23 = 1 << 22;
        /// The group n°24.
        const GROUP_24 = 1 << 23;
        /// The group n°25.
        const GROUP_25 = 1 << 24;
        /// The group n°26.
        const GROUP_26 = 1 << 25;
        /// The group n°27.
        const GROUP_27 = 1 << 26;
        /// The group n°28.
        const GROUP_28 = 1 << 27;
        /// The group n°29.
        const GROUP_29 = 1 << 28;
        /// The group n°30.
        const GROUP_30 = 1 << 29;
        /// The group n°31.
        const GROUP_31 = 1 << 30;
        /// The group n°32.
        const GROUP_32 = 1 << 31;

        /// All of the groups.
        const ALL = u32::MAX;
        /// None of the groups.
        const NONE = 0;
    }
}

impl Default for Group {
    fn default() -> Self {
        Group::ALL
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component, Hash, PartialEq)]
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
                        info!("entry {:?} | {:?} removed", entity_a, entity_b);
                        let Some(_) = collision_records.value.remove(&(entity_a, entity_b)) else {
                            continue;
                        };
                        continue;
                    }
                    _ => {
                        info!("entry {:?} | {:?} added", entity_a, entity_b);
                        collision_records.value.insert((entity_a, entity_b), true);
                    }
                }

                info!(
                    "new collision: distance {:?}\t| radius {:?}",
                    distance,
                    (collider_a.radius + collider_b.radius)
                );
                colliding_entities
                    .entry(entity_a)
                    .or_insert_with(Vec::new)
                    .push(entity_b);
            }
        }
    }

    // //  phase 2: only add new colliders, and remove old collision records
    // for (entity, _, _, _) in query.iter() {
    //     for (mut record_collisions) in collision_records.value.get_mut(&entity) {
    //         for (caught_collisions) in colliding_entities.get(&entity) {
    //             for (record_collision) in record_collisions.iter() {
    //                 for (caught_collision) in caught_collisions.iter() {

    //                 }
    //             }
    //         }
    //     }
    // }

    //  phase 3: update colliders
    for (entity, _, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            // for (collision) in collisions.iter() {
            //     for (existing_entity) in collider.colliding_entities.iter() {

            //     }
            // }
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}

fn handle_collisions<T: Component>(
    mut collision_event_writer: EventWriter<CollisionEvent>,
    // mut collision_records: ResMut<CollisionRecords>,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for &colliding_entity in collider.colliding_entities.iter() {
            //  entity colliding with another entity of the same type
            if query.get(colliding_entity).is_ok() {
                continue;
            }

            info!("collision event written");
            collision_event_writer.send(CollisionEvent::new(entity, colliding_entity));

            // //  if a collision record exists, it means the entities are in currently "colliding" - we only need the single event
            // if let Some(_) = collision_records.value.get(&entity) {
            //     info!("already exists");
            //     continue;
            // } else {
            //     info!("collision started");
            //     //  send collision event

            //     //  create a collision record for this initial collision
            //     collision_records
            //         .value
            //         .insert(entity, CollisionRecord::new(entity, colliding_entity));
            // }
        }
    }
}

pub fn apply_collision_damage(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut attacked_query: Query<&mut Health>,
    attacker_query: Query<&CollisionDamage>,
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
        info!(
            "entity: {:?}\t| health {:?}\tcolliding {:?}\t| damage {:?}",
            entity, health.value, colliding_entity, collision_damage.amount
        );
    }
}

pub fn update_collision_transforms(
    mut collision_event_reader: EventReader<CollisionEvent>,
    attacked_query: Query<(&GlobalTransform, &Collider)>,
    mut attacker_query: Query<(&GlobalTransform, &Collider, &mut Transform, &mut Velocity)>,
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
        info!(
            "deflection: {:?}\t| adjusted_dist {:?}",
            deflection_vec, adjusted_distance
        );
        info!("pre trans {:?}", attacker_transform.translation);
        attacker_transform.translation += deflection_vec * (adjusted_distance + BOUNCE_BUFFER);
        info!("post trans {:?}", attacker_transform.translation);

        //  2: "bounce" the attacker off the attacked
        let radial_velocity = attacker_velocity.value * deflection_vec;
        info!("radial vel {:?}", radial_velocity);
        info!("pre atk vel {:?}", attacker_velocity.value);
        attacker_velocity.value += -(1. + 0.9) * radial_velocity;
        info!("post atk vel {:?}", attacker_velocity.value);
    }
}
