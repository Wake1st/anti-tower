use bevy::prelude::*;

use crate::{health::Health, schedule::InGameSet};

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, attack.in_set(InGameSet::EntityUpdates))
            .add_event::<AttackEvent>();
    }
}

#[derive(Component, Debug)]
pub struct Attack {
    pub amount: f32,
    pub rate: Timer,
}

impl Attack {
    pub fn new(amount: f32, rate: Timer) -> Self {
        Self { amount, rate }
    }
}

#[derive(Event, Debug)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub target: Entity,
}

impl AttackEvent {
    pub fn new(attacker: Entity, target: Entity) -> Self {
        Self { attacker, target }
    }
}

fn attack(
    mut attack_event_reader: EventReader<AttackEvent>,
    mut attacker_query: Query<&mut Attack>,
    mut target_query: Query<&mut Health>,
    time: Res<Time>,
) {
    for &AttackEvent { attacker, target } in attack_event_reader.read() {
        let Ok(mut attack) = attacker_query.get_mut(attacker) else {
            continue;
        };

        attack.rate.tick(time.delta());

        if attack.rate.just_finished() {
            let Ok(mut health) = target_query.get_mut(target) else {
                continue;
            };

            health.value -= attack.amount;
        }
    }
}
