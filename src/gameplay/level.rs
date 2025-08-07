//! Spawn the main level.

use std::str::FromStr;

use bevy::prelude::*;
use nalgebra::Vector2;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    gameplay::tilemap::{TilemapAssets, map_tile},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct GridPosition(Vector2<i32>);

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    tilemap_assets: Res<TilemapAssets>,
) {
    let actions = soukoban::Actions::from_str("R").unwrap();
    let map = soukoban::Map::from_actions(actions).unwrap();

    commands
        .spawn((
            Name::new("Level"),
            Transform::default(),
            Visibility::default(),
            StateScoped(Screen::Gameplay),
            children![(
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            )],
        ))
        .with_children(|commands| {
            for x in 0..map.dimensions().x {
                for y in 0..map.dimensions().y {
                    let position = Vector2::new(x, y);
                    let tiles = map.get(position).unwrap();
                    for tile in tiles.iter() {
                        commands.spawn((map_tile(tile, &tilemap_assets), GridPosition(position)));
                    }
                }
            }
        });
}
