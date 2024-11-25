use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
