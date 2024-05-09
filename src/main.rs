use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bubble::BubblePlugin;
use player::PlayerPlugin;
use potion::PotionPlugin;
use ui::GameUI;

mod bubble;
mod player;
mod potion;
mod ui;

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
                        resolution: (1800.0, 900.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .insert_resource(Money(100.0))
        .add_plugins((PlayerPlugin, BubblePlugin, PotionPlugin, GameUI))
        .run();
}
