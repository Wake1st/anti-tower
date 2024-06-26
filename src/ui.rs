use bevy::{
    prelude::*,
    render::color::Color,
    text::{Text, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        Style, UiRect, Val,
    },
};

use crate::Mana;

pub struct GameUI;

#[derive(Component)]
pub struct ManaText;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui)
            .add_systems(Update, update_mana_ui);
    }
}

fn spawn_game_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(4.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Mana",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                ManaText,
            ));
        });
}

fn update_mana_ui(mut texts: Query<&mut Text, With<ManaText>>, mana: Res<Mana>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Mana: ${:?}", mana.0);
    }
}
