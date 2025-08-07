use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use nalgebra::Vector2;
use soukoban::Tiles;

use crate::{AppSystems, asset_tracking::LoadResource};

#[derive(Component, Deref, DerefMut)]
pub struct GridPosition(pub Vector2<u32>);

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TilemapAssets>();
    app.load_resource::<TilemapAssets>();

    app.add_systems(Update, translate_tiles.in_set(AppSystems::Update));
}

const TILE_SIZE: u32 = 128;

/// Translates the tiles to their grid positions smoothly.
fn translate_tiles(tiles: Query<(&GridPosition, &mut Transform)>) {
    for (grid_position, mut transform) in tiles {
        let target_translation = Vec3::new(
            (grid_position.x * TILE_SIZE) as f32,
            (grid_position.y * TILE_SIZE) as f32,
            transform.translation.z,
        );
        transform.translation = transform.translation.lerp(target_translation, 0.3);
    }
}

pub fn map_tile(tile: Tiles, assets: &TilemapAssets) -> impl Bundle {
    let index = match tile {
        Tiles::Floor => 0,
        Tiles::Wall => 3,
        Tiles::Box => 1,
        Tiles::Goal => 2,
        Tiles::Player => 7,
        _ => panic!(),
    };
    let z = match tile {
        Tiles::Floor | Tiles::Wall => 0.0,
        Tiles::Goal => 1.0,
        Tiles::Box | Tiles::Player => 2.0,
        _ => panic!(),
    };

    (
        Name::new("Tile"),
        Sprite::from_atlas_image(
            assets.atlas.clone(),
            TextureAtlas {
                layout: assets.atlas_layout.clone(),
                index,
            },
        ),
        Transform::from_translation(Vec3::ZERO.with_z(z)),
    )
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct TilemapAssets {
    /// The texture atlas containing the tilemap.
    #[dependency]
    atlas: Handle<Image>,
    /// The layout of the texture atlas.
    atlas_layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for TilemapAssets {
    fn from_world(world: &mut World) -> Self {
        let atlas = {
            let assets = world.resource::<AssetServer>();
            assets.load_with_settings(
                "images/tilemap.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            )
        };
        let atlas_layout = {
            let mut atlas_layout_assets = world.resource_mut::<Assets<TextureAtlasLayout>>();
            atlas_layout_assets.add(TextureAtlasLayout::from_grid(
                UVec2::splat(TILE_SIZE),
                6,
                3,
                Some(UVec2::new(1, 1)),
                None,
            ))
        };

        Self {
            atlas,
            atlas_layout,
        }
    }
}
