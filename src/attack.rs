use bevy::prelude::*;

use crate::{health::Health, movement::Velocity, schedule::InGameSet};

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (attack_occurance, remove_attack_occurances)
                .chain()
                .in_set(InGameSet::EntityUpdates),
        )
        .add_event::<AttackOccuranceDeathEvent>();
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

#[derive(Component, Debug)]
pub struct AttackOccurance {
    pub attacker: Entity,
    pub target: Entity,
}

impl AttackOccurance {
    pub fn new(attacker: Entity, target: Entity) -> Self {
        Self { attacker, target }
    }
}

#[derive(Event, Debug)]
pub struct AttackOccuranceDeathEvent {
    pub target_entity: Entity,
}

impl AttackOccuranceDeathEvent {
    pub fn new(target_entity: Entity) -> Self {
        Self { target_entity }
    }
}

fn attack_occurance(
    occurances: Query<&AttackOccurance>,
    mut attacker_query: Query<(&mut Attack, &mut Velocity), With<AttackOccurance>>,
    mut target_query: Query<&mut Health>,
    time: Res<Time>,
    mut target_death_event_writer: EventWriter<AttackOccuranceDeathEvent>,
) {
    for occurance in occurances.iter() {
        let Ok((mut attack, mut velocity)) = attacker_query.get_mut(occurance.attacker) else {
            continue;
        };

        //  first: stop movement
        velocity.value = Vec3::ZERO;

        //  second: attack at a consistant rate
        attack.rate.tick(time.delta());

        if attack.rate.just_finished() {
            let Ok(mut health) = target_query.get_mut(occurance.target) else {
                continue;
            };

            health.value -= attack.amount;

            if health.value <= 0.0 {
                target_death_event_writer.send(AttackOccuranceDeathEvent::new(occurance.target));
            }
        }
    }
}

fn remove_attack_occurances(
    mut commands: Commands,
    mut target_death_event_reader: EventReader<AttackOccuranceDeathEvent>,
    mut attackers: Query<&mut AttackOccurance>,
) {
    for &AttackOccuranceDeathEvent { target_entity } in target_death_event_reader.read() {
        for occurance in attackers.iter_mut() {
            if occurance.target == target_entity {
                commands
                    .entity(occurance.attacker)
                    .remove::<AttackOccurance>();
            }
        }
    }
}
