use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use soukoban::Tiles;

use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TilemapAssets>();
    app.load_resource::<TilemapAssets>();
}

pub fn map_tile(tile: Tiles, assets: &TilemapAssets) -> impl Bundle {
    let index = match tile {
        Tiles::Floor => 0,
        Tiles::Wall => 1,
        Tiles::Box => 2,
        Tiles::Goal => 3,
        Tiles::Player => 4,
        _ => panic!(),
    };
    let z = match tile {
        Tiles::Floor | Tiles::Wall | Tiles::Goal => 0.,
        Tiles::Box | Tiles::Player => 1.,
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
        Transform::from_translation(Vec3::new(0., 0., z)),
    )
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct TilemapAssets {
    #[dependency]
    atlas: Handle<Image>,
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
                UVec2::new(128, 128),
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
