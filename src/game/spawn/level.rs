//! Spawn the main level by triggering other observers.
use bevy::prelude::*;

// use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/level.glb")),
        ..default()
    });
}
