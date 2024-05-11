use bevy::prelude::*;

pub struct FootmanPlugin;

impl Plugin for FootmanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_footman);
    }
}

#[derive(Component)]
pub struct Footman {
    pub health: f32,
}

fn spawn_footman(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("footman.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Footman { health: 10.0 },
        Name::new("Footman"),
    ));
}
