use bevy::prelude::*;

use crate::{
    collisions::{Collider, CollisionGroups},
    detection::{DetectionGroups, Target},
    group::Group,
    health::Health,
    player::Player,
    schedule::InGameSet,
    Mana,
};

const COST: f32 = 100.0;
const HEALTH: f32 = 200.0;
const COLLIDER_RADIUS: f32 = 48.0;
const BASE_GENERATION_RATE: f32 = 4.0;
const MAX_MANA: f32 = 100.0;
const DRAIN_RADIUS: f32 = 140.0;
const DRAIN_RATE: f32 = 60.0;

pub struct HarvesterPlugin;

impl Plugin for HarvesterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_harvester.in_set(InGameSet::UserInput))
            .add_systems(
                Update,
                (generate_mana, drain_mana)
                    .chain()
                    .in_set(InGameSet::EntityUpdates),
            );
    }
}

#[derive(Component, Debug)]
pub struct Harvester {
    mana: f32,
    generation_rate: f32,
}

impl Harvester {
    pub fn new(mana: f32, generation_rate: f32) -> Self {
        Self {
            mana,
            generation_rate,
        }
    }
}

fn spawn_harvester(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut mana: ResMut<Mana>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if mana.0 >= COST {
        mana.0 -= COST;

        let texture: Handle<Image> = asset_server.load("harvester.png");

        commands.spawn((
            SpriteBundle {
                texture,
                ..default()
            },
            Collider::new(COLLIDER_RADIUS),
            CollisionGroups::new(Group::ALLY, Group::NONE),
            DetectionGroups::new(Group::ALLY, Group::NONE),
            Target,
            Health::new(HEALTH),
            Harvester::new(0.0, BASE_GENERATION_RATE),
            Name::new("Harvester"),
        ));
    }
}

fn generate_mana(time: Res<Time>, mut harvesters: Query<&mut Harvester>) {
    for mut harvester in &mut harvesters {
        if harvester.mana < MAX_MANA {
            harvester.mana += harvester.generation_rate * time.delta_seconds();

            if harvester.mana > MAX_MANA {
                harvester.mana = MAX_MANA;
            }
        }
    }
}

fn drain_mana(
    mut harvesters: Query<(&GlobalTransform, &mut Harvester)>,
    player: Query<&GlobalTransform, With<Player>>,
    time: Res<Time>,
    mut mana: ResMut<Mana>,
) {
    let Ok(player_transform) = player.get_single() else {
        return;
    };

    for (harvester_transform, mut harvester) in &mut harvesters {
        let distance = player_transform
            .translation()
            .distance(harvester_transform.translation());

        if distance < DRAIN_RADIUS {
            let drainable_mana = DRAIN_RATE * time.delta_seconds();

            if harvester.mana < drainable_mana {
                mana.0 += harvester.mana;
                harvester.mana = 0.0;
            } else {
                mana.0 += drainable_mana;
                harvester.mana -= drainable_mana;
            };
        }
    }
}
