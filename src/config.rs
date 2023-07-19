use std::sync::{LazyLock, RwLock};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs::{read_to_string, File};
use serde_json::{from_str, to_string};
use std::io::Write;
use crate::utils::CowStr;

pub static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| {
    let mut config = Config::default();

    if Path::new("config.json").exists() {
        let unparsed = read_to_string("config.json").unwrap();
        config = from_str(&unparsed).unwrap();
    } else {
        save_config(&config).unwrap();
    }

    RwLock::new(config)
});

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub web_server: WebServerConfig,
    pub database: DatabaseConfig,
    pub secrets: SecretsConfig,
}

impl Default for Config {
    fn default() -> Config  {
        Config {
            web_server: WebServerConfig::default(),
            database: DatabaseConfig::default(),
            secrets: SecretsConfig::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WebServerConfig {
    pub address: CowStr,
    pub port: u16,
}

impl Default for WebServerConfig {
    fn default() -> WebServerConfig  {
        WebServerConfig {
            address: "localhost".into(),
            port: 8080,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub address: CowStr,
    pub port: u16,
    pub username: CowStr,
    pub password: CowStr,
}

impl Default for DatabaseConfig {
    fn default() -> DatabaseConfig  {
        DatabaseConfig {
            address: "localhost".into(),
            port: 8000,
            username: "root".into(),
            password: "root".into(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SecretsConfig {
    pub jwt: CowStr,
}

impl Default for SecretsConfig {
    fn default() -> SecretsConfig {
        SecretsConfig {
            jwt: "ChangeThisSecret".into(),
        }
    }
}

pub fn save_current() -> std::io::Result<()> {
    save_config(&*CONFIG.read().unwrap())
}

pub fn save_config(config: &Config) -> std::io::Result<()> {
    let parsed = to_string(config).unwrap();
    let mut f = File::create("config.json")?;
    f.write_all(parsed.as_bytes())?;
    Ok(())
}
