use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sl_meta::json::jre_manifest::JreManifestDownloadType;
use sl_utils::utils::errors::BackendError;
use velcro::hash_map_from;

use crate::{json::jre_manifest::download_jre_manifest_version, JAVAS_DIR, LAUNCHER_DIR};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config(HashMap<String, String>);

impl Config {
    fn create_default_global() -> Result<Self, std::io::Error> {
        Ok(Self(hash_map_from! {
            "auth_player_name": "synther",
            "auth_access_token": "0",
        }))
    }

    fn global_config_path() -> PathBuf {
        LAUNCHER_DIR.join("config.json")
    }

    pub fn read_global() -> Result<Self, std::io::Error> {
        let path = Self::global_config_path();

        let config = if !path.exists() {
            let config = Self::create_default_global()?;
            let file = File::create(path)?;
            serde_json::to_writer_pretty(file, &config).unwrap();
            config
        } else {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap()
        };

        Ok(config)
    }
}

impl Config {
    pub fn new(map: HashMap<String, String>) -> Self {
        Self(map)
    }

    pub async fn create_config(component: &str) -> Result<Self, BackendError> {
        let java_path = JAVAS_DIR.join(component);

        let java_binary = if cfg!(target_os = "windows") {
            "java.exe"
        } else {
            "java"
        };

        if java_path.exists() {
            return Ok(Self(hash_map_from! {
                "java": java_path.join("bin").join(java_binary).to_string_lossy().to_string()
            }));
        }

        let jre_manifest_download_type = JreManifestDownloadType::from(component);
        download_jre_manifest_version(jre_manifest_download_type).await?;

        Ok(Self(hash_map_from! {
            "java": java_path.join("bin").join(java_binary).to_string_lossy().to_string()
        }))
    }

    pub fn empty() -> Self {
        Self(HashMap::new())
    }

    pub fn update_config_field(
        &mut self,
        key: &str,
        new_value: &str,
    ) -> Result<(), std::io::Error> {
        let config_path = LAUNCHER_DIR.join("config.json");

        let config_data = fs::read_to_string(&config_path)?;
        let mut json: Value = serde_json::from_str(&config_data)?;

        if let Some(obj) = json.as_object_mut() {
            obj.insert(key.to_string(), Value::String(new_value.to_string()));
        }

        fs::write(&config_path, serde_json::to_string_pretty(&json)?)?;
        Ok(())
    }

    pub fn get(&self, entry: &str) -> Option<&str> {
        self.0.get(entry).map(|x| x.as_str())
    }

    pub fn merge(self, mut other: Self) -> Self {
        other.0.extend(self.0);
        other
    }
}
