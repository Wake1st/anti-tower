use bevy::prelude::*;

use crate::{health::Health, movement::Velocity, schedule::InGameSet};

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, attack.in_set(InGameSet::EntityUpdates));
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

fn attack(
    // mut attack_event_reader: EventReader<AttackEvent>,
    mut commmands: Commands,
    occurances: Query<&AttackOccurance>,
    mut attacker_query: Query<(&mut Attack, &mut Velocity), With<AttackOccurance>>,
    mut target_query: Query<&mut Health>,
    time: Res<Time>,
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
                commmands
                    .entity(occurance.attacker)
                    .remove::<AttackOccurance>();
            }
        }
    }
}
