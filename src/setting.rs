use bevy::asset::ron::de::from_reader;
use bevy::asset::ron::ser::{to_string_pretty, PrettyConfig};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub trait GameSetting {
    const DEFAULT_CONF: &'static str = "game_setting.conf";

    fn load(&mut self) -> anyhow::Result<()> {
        let config_path = PathBuf::from(Self::DEFAULT_CONF);
        self.load_from(&config_path)
    }

    fn load_from(&mut self, config_path: &PathBuf) -> anyhow::Result<()> {
        let file = File::open(config_path)?;
        self = from_reader(file)?;
        Ok(())
    }

    fn save(&self) -> anyhow::Result<()> {
        let config_path = PathBuf::from(Self::DEFAULT_CONF);
        self.save_to(&config_path)
    }

    fn save_to(&self, config_path: &PathBuf) -> anyhow::Result<()> {
        let pretty = PrettyConfig::default();
        let ron_str = to_string_pretty(self, pretty)?;

        if let Some(parent_dir) = config_path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }
        let mut file = File::create(config_path)?;
        file.write_all(ron_str.as_bytes()).map_err(|e| anyhow::anyhow!(e))
    }
}
