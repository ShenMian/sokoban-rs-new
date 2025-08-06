use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TilemapAssets>();
    app.load_resource::<TilemapAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct TilemapAssets {
    #[dependency]
    atlas: Handle<Image>,
    atlas_layout: TextureAtlasLayout,
}

impl FromWorld for TilemapAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            atlas: assets.load_with_settings(
                "images/tilemap.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            atlas_layout: TextureAtlasLayout::from_grid(
                UVec2::new(128, 128),
                6,
                3,
                Some(UVec2::new(1, 1)),
                None,
            ),
        }
    }
}
