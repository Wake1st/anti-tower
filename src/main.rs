mod bubble;
mod camera;
mod collisions;
mod despawn;
mod detection;
mod footman;
mod health;
mod movement;
mod player;
mod schedule;
mod state;
mod ui;

use bevy::prelude::*;

use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bubble::BubblePlugin;
use camera::CameraPlugin;
use collisions::CollisionsPlugin;
use despawn::DespawnPlugin;
use detection::DetectionPlugin;
use footman::FootmanPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;
use ui::GameUI;

const PIXELS_PER_METER: f32 = 32.0;

#[derive(Resource)]
pub struct Money(pub f32);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Learning Bevy".into(),
                        position: WindowPosition::At(IVec2 { x: 50, y: 70 }),
                        resolution: (1800.0, 900.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        // .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)))
        .insert_resource(Money(100.0))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((
            SchedulePlugin,
            StatePlugin,
            DetectionPlugin,
            MovementPlugin,
            CollisionsPlugin,
            DespawnPlugin,
            CameraPlugin,
            PlayerPlugin,
            BubblePlugin,
            FootmanPlugin,
            GameUI,
        ))
        .run();
}
