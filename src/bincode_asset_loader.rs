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

#[derive(Default)]
pub struct BincodeLoaderPlugin<T>
where
    T: Asset,
{
    _unused: Option<T>,
}

impl<T> Plugin for BincodeLoaderPlugin<T>
where
    T: Asset + Default + for<'de> Deserialize<'de>,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<T>().init_asset_loader::<BincodeAssetLoader<T>>();
    }
}

#[derive(TypePath, Default)]
struct BincodeAssetLoader<T>
where
    T: Asset,
{
    _unused: Option<T>,
}

#[derive(Debug, Error)]
enum BincodeAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    DecodeError(#[from] postcard::Error),
}

impl<T> AssetLoader for BincodeAssetLoader<T>
where
    T: Asset + TypePath + for<'de> Deserialize<'de>,
{
    type Asset = T;
    type Settings = ();
    type Error = BincodeAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let custom_asset = postcard::from_bytes(bytes.as_slice())?;
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["custom"]
    }
}
