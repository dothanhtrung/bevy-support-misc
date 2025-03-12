use bevy::app::App;
use bevy::prelude::{on_event, warn, Event, IntoSystemConfigs, Plugin, Res, ResMut, Resource, Startup, Update};
use bevy::tasks::IoTaskPool;
use serde::{Deserialize, Serialize};
use simple_crypt::{decrypt, encrypt};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct EncryptSavePlugin<T>
where
    T: Resource + Default + EncryptSave + Clone,
{
    config: T,
}

impl<T> Plugin for EncryptSavePlugin<T>
where
    T: Resource + Default + EncryptSave + Clone,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.clone())
            .add_event::<SaveEncrypt>()
            .add_systems(Startup, load::<T>)
            .add_systems(Update, save::<T>.run_if(on_event::<SaveEncrypt>));
    }
}

impl<T> EncryptSavePlugin<T>
where
    T: Resource + Default + EncryptSave + Clone,
{
    pub fn new(config: T) -> Self {
        Self { config }
    }
}

#[derive(Event)]
pub struct SaveEncrypt;

fn load<T>(mut data: ResMut<T>)
where
    T: Resource + EncryptSave,
{
    if let Err(e) = data.load() {
        warn!("Failed to load save data: {}", e);
    }
}

fn save<T>(data: Res<T>)
where
    T: Resource + EncryptSave,
{
    if let Err(e) = data.save() {
        warn!("Failed to save game: {}", e);
    }
}

pub trait EncryptSave: Serialize + for<'de> Deserialize<'de> {
    const DEFAULT_SAVE: &'static str = "default_save.dat";
    const ENCR_KEY: &'static str = "0123456789abcdef";

    fn load(&mut self) -> anyhow::Result<()> {
        let config_path = PathBuf::from(Self::DEFAULT_SAVE);
        self.load_from(&config_path)
    }

    fn load_from(&mut self, config_path: &PathBuf) -> anyhow::Result<()> {
        let enc_saved = std::fs::read(config_path)?;
        let decrypted = decrypt(enc_saved.as_slice(), Self::ENCR_KEY.as_bytes())?;
        (*self, _) = bincode::serde::decode_from_slice(decrypted.as_slice(), bincode::config::legacy())?;
        Ok(())
    }

    fn save(&self) -> anyhow::Result<()> {
        let config_path = PathBuf::from(Self::DEFAULT_SAVE);
        self.save_to(config_path)
    }

    fn save_to(&self, saved_path: PathBuf) -> anyhow::Result<()> {
        let data = bincode::serde::encode_to_vec(self, bincode::config::legacy())?;
        let enc_saved = encrypt(data.as_slice(), Self::ENCR_KEY.as_bytes())?;

        #[cfg(not(target_arch = "wasm32"))]
        IoTaskPool::get()
            .spawn(async move {
                if let Some(parent_dir) = saved_path.parent() {
                    std::fs::create_dir_all(parent_dir)?;
                }
                File::create(saved_path).and_then(|mut file| file.write_all(enc_saved.as_slice()))
            })
            .detach();

        Ok(())
    }
}
