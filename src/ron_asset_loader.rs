use bevy::app::App;
use bevy::asset::io::Reader;
use bevy::asset::{
    AssetLoader,
    LoadContext,
};
use bevy::prelude::{
    Asset,
    AssetApp,
    Plugin,
    TypePath,
};
use serde::Deserialize;
use thiserror::Error;

pub struct RonLoaderPlugin<T>
where
    T: Asset + TypePath + for<'de> Deserialize<'de>,
{
    _unused: Option<T>,
}

impl<T> Plugin for RonLoaderPlugin<T>
where
    T: Asset + TypePath + Default + for<'de> Deserialize<'de>,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<T>().init_asset_loader::<RonAssetLoader<T>>();
    }
}

#[derive(TypePath, Default)]
struct RonAssetLoader<T>
where
    T: Asset + TypePath + for<'de> Deserialize<'de>,
{
    _unused: Option<T>,
}

#[derive(Debug, Error)]
enum RonAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl<T> AssetLoader for RonAssetLoader<T>
where
    T: Asset + TypePath + for<'de> Deserialize<'de>,
{
    type Asset = T;
    type Settings = ();
    type Error = RonAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let custom_asset = ron::de::from_bytes::<T>(&bytes)?;
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["custom"]
    }
}
