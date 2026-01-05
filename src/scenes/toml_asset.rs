use bevy::asset::{io::Reader, Asset, AssetLoader, LoadContext};
use bevy::reflect::TypePath;
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Clone, Deserialize)]
pub struct TomlAsset(pub String);

#[derive(Default)]
pub struct TomlAssetLoader;

#[derive(Debug, Error)]
pub enum TomlAssetError {
    #[error("Failed to read TOML asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML asset is not valid UTF-8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

impl AssetLoader for TomlAssetLoader {
    type Asset = TomlAsset;
    type Settings = ();
    type Error = TomlAssetError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let text = String::from_utf8(bytes)?;
        Ok(TomlAsset(text))
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
