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
pub struct BinLoaderPlugin<T>
where
    T: Asset,
{
    _unused: Option<T>,
}

impl<T> Plugin for BinLoaderPlugin<T>
where
    T: Asset + Default + for<'de> Deserialize<'de>,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<T>().init_asset_loader::<BinAssetLoader<T>>();
    }
}

#[derive(TypePath, Default)]
struct BinAssetLoader<T>
where
    T: Asset,
{
    _unused: Option<T>,
    encrypt_key: String,
}

#[derive(Debug, Error)]
enum BinAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse blob: {0}")]
    DecodeError(#[from] postcard::Error),
}

impl<T> AssetLoader for BinAssetLoader<T>
where
    T: Asset + TypePath + for<'de> Deserialize<'de>,
{
    type Asset = T;
    type Settings = ();
    type Error = BinAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        if !self.encrypt_key.is_empty() {
            bytes = simple_crypt::decrypt(bytes.as_slice(), self.encrypt_key.as_bytes())
                .map_err(|_| BinAssetLoaderError::DecodeError(postcard::Error::SerdeDeCustom))?;
        };

        let custom_asset = postcard::from_bytes(bytes.as_slice())?;
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["bin", "dat"]
    }
}
