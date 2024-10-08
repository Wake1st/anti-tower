mod attack;
mod bubble;
mod camera;
mod collisions;
mod despawn;
mod detection;
mod footman;
mod group;
mod harvester;
mod health;
mod movement;
mod player;
mod schedule;
mod state;
mod tower;
mod ui;

use bevy::prelude::*;

use attack::AttackPlugin;
use bubble::BubblePlugin;
use camera::CameraPlugin;
use collisions::CollisionsPlugin;
use despawn::DespawnPlugin;
use detection::DetectionPlugin;
use footman::FootmanPlugin;
use harvester::HarvesterPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;
use tower::TowerPlugin;
use ui::GameUI;

#[derive(Resource)]
pub struct Mana(pub f32);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Anti-Tower".into(),
                        position: WindowPosition::At(IVec2 { x: 50, y: 70 }),
                        resolution: (1800.0, 900.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        // .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Tab)))
        .insert_resource(Mana(100.0))
        .add_plugins((
            SchedulePlugin,
            StatePlugin,
            DetectionPlugin,
            MovementPlugin,
            CollisionsPlugin,
            AttackPlugin,
            DespawnPlugin,
            CameraPlugin,
            PlayerPlugin,
            HarvesterPlugin,
            BubblePlugin,
            FootmanPlugin,
            TowerPlugin,
            GameUI,
        ))
        .run();
}
