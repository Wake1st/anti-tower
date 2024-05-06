use bevy::prelude::*;

use crate::{Money, Player};
pub struct PotionPlugin;

impl Plugin for PotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_potion_parent)
            .add_systems(Update, (spawn_potion, potion_lifetime))
            .register_type::<Potion>();
    }
}

#[derive(Component)]
pub struct PotionParent;

fn spawn_potion_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        PotionParent,
        Name::new("PotionParent"),
    ));
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Potion {
    pub lifetime: Timer,
}

fn spawn_potion(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<PotionParent>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();
    let parent = parent.single();

    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent $10 on a potion; remaining money: ${:?}", money.0);

        let texture = asset_server.load("potion.png");

        commands.entity(parent).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: *player_transform,
                    ..default()
                },
                Potion {
                    lifetime: Timer::from_seconds(2.0, TimerMode::Once),
                },
                Name::new("Potion"),
            ));
        });
    }
}

fn potion_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut potions: Query<(Entity, &mut Potion)>,
    parent: Query<Entity, With<PotionParent>>,
    mut money: ResMut<Money>,
) {
    let parent = parent.single();

    for (potion_entity, mut potion) in &mut potions {
        potion.lifetime.tick(time.delta());

        if potion.lifetime.finished() {
            money.0 += 15.0;
            commands.entity(parent).remove_children(&[potion_entity]);
            commands.entity(potion_entity).despawn();
            info!("Potion sold for $15; current money: ${:?}", money.0);
        }
    }
}
