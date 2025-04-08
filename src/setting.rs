use bevy::app::App;
use bevy::asset::ron::de::from_reader;
use bevy::asset::ron::ser::{to_string_pretty, PrettyConfig};
use bevy::prelude::{on_event, warn, Event, IntoSystemConfigs, Plugin, Res, ResMut, Resource, Startup, Update};
use bevy::tasks::IoTaskPool;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct GameSettingSupportPlugin<T>
where
    T: Resource + Default + GameSetting + Clone,
{
    config: T,
}

impl<T> Plugin for GameSettingSupportPlugin<T>
where
    T: Resource + Default + GameSetting + Clone,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.clone())
            .add_event::<GameSettingChanged>()
            .add_systems(Startup, load_config::<T>)
            .add_systems(Update, save_config::<T>.run_if(on_event::<GameSettingChanged>));
    }
}

impl<T> GameSettingSupportPlugin<T>
where
    T: Resource + Default + GameSetting + Clone,
{
    pub fn new(config: T) -> Self {
        Self { config }
    }
}

#[derive(Event)]
pub struct GameSettingChanged;

fn load_config<T>(mut config: ResMut<T>)
where
    T: Resource + GameSetting,
{
    if let Err(e) = config.load() {
        warn!(
            "Failed to load game config {} : {}",
            T::config_path().as_path().to_str().unwrap_or_default(),
            e
        );
    }
}

fn save_config<T>(config: Res<T>)
where
    T: Resource + GameSetting,
{
    if let Err(e) = config.save() {
        warn!(
            "Failed to save game config {}: {}",
            T::config_path().as_path().to_str().unwrap_or_default(),
            e
        );
    }
}

pub trait GameSetting: Serialize + for<'de> Deserialize<'de> {
    const DEFAULT_CONF: &'static str = "game_setting.conf";

    fn config_path() -> PathBuf {
        if cfg!(target_os = "android") {
            // It should be /data/data/com.yourapp.package/setting.txt
            PathBuf::from(Self::DEFAULT_CONF)
        } else if let Some(data_local_dir) = dirs::data_local_dir() {
            data_local_dir.join(Self::DEFAULT_CONF)
        } else {
            PathBuf::from(Self::DEFAULT_CONF)
        }
    }

    fn load(&mut self) -> anyhow::Result<()> {
        self.load_from(&Self::config_path())
    }

    fn load_from(&mut self, config_path: &PathBuf) -> anyhow::Result<()> {
        let file = File::open(config_path)?;
        *self = from_reader(file)?;
        Ok(())
    }

    fn save(&self) -> anyhow::Result<()> {
        self.save_to(Self::config_path())
    }

    fn save_to(&self, config_path: PathBuf) -> anyhow::Result<()> {
        let pretty = PrettyConfig::default();
        let ron_str = to_string_pretty(self, pretty)?;

        #[cfg(not(target_arch = "wasm32"))]
        IoTaskPool::get()
            .spawn(async move {
                if let Some(parent_dir) = config_path.parent() {
                    std::fs::create_dir_all(parent_dir)?;
                }
                let mut file = File::create(config_path)?;
                file.write_all(ron_str.as_bytes()).map_err(|e| anyhow::anyhow!(e))
            })
            .detach();

        Ok(())
    }
}
