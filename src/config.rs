use serde::{Deserialize, Serialize};
use config::Config;
use anyhow::Result;

use once_cell::sync::OnceCell;

static MYCONFIG: OnceCell<MyConfig> = OnceCell::new();

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MyConfig {
    #[serde(default="MyConfig::default_port")]
    port: u16
}

impl MyConfig {
    fn default_port() -> u16 {
        8080
    }
}

pub fn set_config(cfg: MyConfig) -> Result<()> {
    let _ = MYCONFIG.set(cfg);
    Ok(())
}

pub fn get_config_opt() -> Option<&'static MyConfig> {
    MYCONFIG.get()
}

pub fn get_config() -> &'static MyConfig {
    MYCONFIG.get().unwrap()
}

pub fn init() -> Result<()> {
    let config_path = format!(
        "{}/.config/any2cast.toml",
        std::env::var("HOME").expect("HOME env should exist")
    );

    let settings = Config::builder()
        .add_source(config::File::with_name(&config_path))
        .add_source(config::Environment::with_prefix("ANY2CAST"))
        .build()?;

    let c = settings.try_deserialize::<MyConfig>()?;

    set_config(c).unwrap();

    Ok(())
}
