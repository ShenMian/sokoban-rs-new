//! Spawn the main level.

use bevy::prelude::*;
use nalgebra::Vector2;

use super::tilemap::GridPosition;
use crate::{
    gameplay::tilemap::{TilemapAssets, map_tile},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    setup_database(app.world_mut().commands());
}

#[derive(Resource, Deref, DerefMut)]
pub struct Database(Vec<soukoban::Level>);

fn setup_database(mut commands: Commands) {
    let mut levels = Vec::new();
    let level_path = std::env::current_dir().unwrap().join("assets/levels");
    for path in std::fs::read_dir(level_path).unwrap() {
        let path = path.unwrap().path();
        levels.extend(
            soukoban::Level::load_from_str(&std::fs::read_to_string(path).unwrap())
                .filter_map(Result::ok),
        );
    }
    commands.insert_resource(Database(levels));
}

#[derive(Component)]
struct Level {
    inner: soukoban::Level,
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    tilemap_assets: Res<TilemapAssets>,
    database: Res<Database>,
) {
    let level = database.first().unwrap().clone();
    let map = level.map();
    commands
        .spawn((
            Name::new("Level"),
            Level {
                inner: level.clone(),
            },
            Transform::default(),
            Visibility::default(),
            StateScoped(Screen::Gameplay),
        ))
        .with_children(|commands| {
            for x in 0..map.dimensions().x {
                for y in 0..map.dimensions().y {
                    let position = Vector2::new(x, y);
                    let tiles = map.get(position).unwrap();
                    for tile in tiles.iter() {
                        commands.spawn((
                            map_tile(tile, &tilemap_assets),
                            GridPosition(position.map(|x| x as u32)),
                        ));
                    }
                }
            }
        });
}
