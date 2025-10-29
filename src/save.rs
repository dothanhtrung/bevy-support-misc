use crate::setting::{
    GameSetting,
    GameSettingSupportPlugin,
};
use bevy::app::App;
#[cfg(feature = "log")]
use bevy::prelude::{
    error,
    warn,
};
use bevy::prelude::{
    on_message,
    Deref,
    DerefMut,
    IntoScheduleConfigs,
    Message,
    MessageReader,
    Plugin,
    Res,
    ResMut,
    Resource,
    Update,
};
use bevy::tasks::IoTaskPool;
use serde::{
    Deserialize,
    Serialize,
};
use simple_crypt::{
    decrypt,
    encrypt,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{
    Path,
    PathBuf,
};

#[derive(Default)]
pub struct EncryptSavePlugin<T>
where
    T: Resource + Default + EncryptSave + Clone,
{
    _config: Option<T>,
}

impl<T> Plugin for EncryptSavePlugin<T>
where
    T: Resource + Default + EncryptSave + Clone,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(GameSettingSupportPlugin::<SaveConfig>::default())
            .insert_resource(T::default())
            .insert_resource(CurrentSave(0))
            .add_message::<SaveGame>()
            .add_systems(Update, load::<T>.run_if(on_message::<LoadGame>))
            .add_systems(Update, save::<T>.run_if(on_message::<SaveGame>));
    }
}

#[derive(Message, Deref, DerefMut)]
pub struct SaveGame(pub bool); // new or overwrite

#[derive(Message, Deref, DerefMut)]
pub struct LoadGame(pub u32);

#[derive(Resource, Deref, DerefMut)]
pub struct CurrentSave(pub u32);

#[derive(Resource, Deserialize, Serialize, Clone, Default)]
pub struct SaveConfig {
    saves: HashMap<u32, PathBuf>,
    save_dir: PathBuf,
}

impl GameSetting for SaveConfig {
    const DEFAULT_CONF: &'static str = "save_setting.conf";
}

fn load<T>(
    mut data: ResMut<T>,
    mut load_message: MessageReader<LoadGame>,
    mut current_save: ResMut<CurrentSave>,
    save_config: Res<SaveConfig>,
) where
    T: Resource + EncryptSave,
{
    for id in load_message.read() {
        if let Some(saved_path) = save_config.saves.get(&id.0) {
            let saved_path = save_config.save_dir.join(saved_path);
            if let Err(_e) = data.load_from(&saved_path) {
                #[cfg(feature = "log")]
                warn!("Failed to load save data {}: {}", saved_path.display(), _e);
            } else {
                current_save.0 = id.0;
            }
        }
    }
}

fn save<T>(
    data: Res<T>,
    mut save_message: MessageReader<SaveGame>,
    current_save: Res<CurrentSave>,
    save_config: Res<SaveConfig>,
) where
    T: Resource + EncryptSave,
{
    for save in save_message.read() {
        let new_save = save.0;
        if new_save {
            let file_name = format!("{}.dat", random_string());
            let saved_path = save_config.save_dir.join(file_name.as_str());
            if let Err(_e) = data.save_to(saved_path.clone()) {
                #[cfg(feature = "log")]
                error!("Failed to save data {}: {}", saved_path.display(), _e);
            }
        } else {
            if let Some(saved_path) = save_config.saves.get(&current_save.0) {
                let saved_path = save_config.save_dir.join(saved_path);
                if let Err(_e) = data.save_to(saved_path.clone()) {
                    #[cfg(feature = "log")]
                    error!("Failed to save data {}: {}", saved_path.display(), _e);
                }
            }
        }
    }
}

pub trait EncryptSave: Serialize + for<'de> Deserialize<'de> {
    const ENCR_KEY: &'static str = "0123456789abcdef";

    fn load_from(&mut self, config_path: &Path) -> anyhow::Result<()> {
        let enc_saved = std::fs::read(config_path)?;
        let decrypted = decrypt(enc_saved.as_slice(), Self::ENCR_KEY.as_bytes())?;
        (*self, _) = bincode::serde::decode_from_slice(decrypted.as_slice(), bincode::config::legacy())?;
        Ok(())
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

fn random_string() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const LEN: usize = 12;

    (0..LEN)
        .map(|_| {
            let idx = fastrand::usize(..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
