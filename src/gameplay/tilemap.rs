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
    floor: Handle<Image>,
    #[dependency]
    wall: Handle<Image>,
    #[dependency]
    r#box: Handle<Image>,
    #[dependency]
    goal: Handle<Image>,
    #[dependency]
    player: Handle<Image>,
}

impl FromWorld for TilemapAssets {
    fn from_world(world: &mut World) -> Self {
        let settings = |settings: &mut ImageLoaderSettings| {
            // Use `nearest` image sampling to preserve pixel art style.
            settings.sampler = ImageSampler::nearest();
        };

        let assets = world.resource::<AssetServer>();
        Self {
            floor: assets.load_with_settings("images/floor.png", settings),
            wall: assets.load_with_settings("images/wall.png", settings),
            r#box: assets.load_with_settings("images/box.png", settings),
            goal: assets.load_with_settings("images/goal.png", settings),
            player: assets.load_with_settings("images/player.png", settings),
        }
    }
}
