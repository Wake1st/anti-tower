use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Health {
    value: f32,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}
