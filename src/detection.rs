use bevy::prelude::*;

use crate::{
    attack::AttackOccurance,
    bubble::{Bubble, BubbleSpawner},
    footman::Footman,
    group::Group,
    schedule::InGameSet,
    tower::Tower,
};

const ATTACK_RANGE: f32 = 40.0;

pub struct DetectionPlugin;

impl Plugin for DetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                detect::<Bubble, Tower>,
                detect::<Bubble, Footman>,
                detect::<Footman, Bubble>,
                detect::<Footman, BubbleSpawner>,
            )
                .in_set(InGameSet::EntityUpdates),
        )
        .add_event::<DetectionEvent>();
    }
}

#[derive(Component, Debug)]
pub struct Tracker {
    pub vision: f32,
}

impl Tracker {
    pub fn new(vision: f32) -> Self {
        Self { vision }
    }
}

#[derive(Component, Debug)]
pub struct Target;
//  TODO: add cloaking, a 0..=1 value to reduce the tracker vision by

#[derive(Event, Debug)]
pub struct DetectionEvent {
    pub tracker_entity: Entity,
    pub target_entity: Entity,
}

impl DetectionEvent {
    pub fn new(tracker_entity: Entity, target_entity: Entity) -> Self {
        Self {
            tracker_entity,
            target_entity,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Component)]
pub struct DetectionGroups {
    /// Groups memberships.
    pub memberships: Group,
    /// Groups filter.
    pub filters: Group,
}

impl DetectionGroups {
    /// Creates a new collision-groups with the given membership masks and filter masks.
    pub const fn new(memberships: Group, filters: Group) -> Self {
        Self {
            memberships,
            filters,
        }
    }
}

fn detect<T: Component, U: Component>(
    mut commands: Commands,
    trackers: Query<
        (
            Entity,
            &DetectionGroups,
            &Tracker,
            &GlobalTransform,
            Option<&AttackOccurance>,
        ),
        With<T>,
    >,
    targets: Query<(Entity, &DetectionGroups, &GlobalTransform), With<U>>,
    mut tracking_event_writer: EventWriter<DetectionEvent>,
) {
    for (tracker_entity, tracker_groups, tracker, tracker_transform, attack_option) in
        trackers.iter()
    {
        //  no need to track if there is attack
        if let Some(_) = attack_option {
            continue;
        }

        for (target_entity, target_groups, target_transform) in targets.iter() {
            if tracker_entity == target_entity {
                continue;
            }

            //  first, check the groups for a match - [fastest check(?) should be first]
            if (tracker_groups.filters & target_groups.memberships) == Group::NONE {
                continue;
            }

            let distance = tracker_transform
                .translation()
                .distance(target_transform.translation());

            if distance == 0.0 {
                continue;
            }

            match distance {
                i if i < ATTACK_RANGE => {
                    commands
                        .entity(tracker_entity)
                        .insert(AttackOccurance::new(tracker_entity, target_entity));
                }
                i if i < tracker.vision => {
                    tracking_event_writer.send(DetectionEvent::new(tracker_entity, target_entity));
                }
                _ => continue,
            }
        }
    }
}
