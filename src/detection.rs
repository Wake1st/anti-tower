use bevy::prelude::*;

use crate::schedule::InGameSet;

pub struct DetectionPlugin;

impl Plugin for DetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect.in_set(InGameSet::EntityUpdates))
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

fn detect(
    trackers: Query<(Entity, &Tracker, &GlobalTransform)>,
    targets: Query<(Entity, &GlobalTransform), With<Target>>,
    mut tracking_event_writer: EventWriter<DetectionEvent>,
) {
    for (tracker_entity, tracker, tracker_transform) in trackers.iter() {
        for (target_entity, target_transform) in targets.iter() {
            if tracker_entity == target_entity {
                continue;
            }

            let distance = tracker_transform
                .translation()
                .distance(target_transform.translation());

            if distance == 0.0 {
                continue;
            }

            if distance < tracker.vision {
                tracking_event_writer.send(DetectionEvent::new(tracker_entity, target_entity));
            }
        }
    }
}
