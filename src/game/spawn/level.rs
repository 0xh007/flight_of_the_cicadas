//! Spawn the main level by triggering other observers.
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_resource::PrimitiveTopology;

use bevy::prelude::*;

use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnPlayer);
}

fn create_hexagon_mesh(size: f32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let vertices = (0..6)
        .map(|i| {
            let angle = std::f32::consts::PI / 3.0 * i as f32;
            [size * angle.cos(), size * angle.sin(), 0.0]
        })
        .collect::<Vec<_>>();

    // Center vertex
    let mut positions = vec![[0.0, 0.0, 0.0]];
    positions.extend(vertices);

    let indices = vec![0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1];

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}
