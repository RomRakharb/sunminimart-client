use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct Setting {
    pub database_url: String,
}

impl Setting {
    pub fn get_setting() -> std::io::Result<Self> {
        let mut file = File::open("setting.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let setting: Setting = serde_json::from_str(&contents)?;
        Ok(setting)
    }

    pub fn set_setting(&self) -> std::io::Result<()> {
        let setting = serde_json::to_string(self)?;
        let mut file = File::create("setting.json")?;
        file.write_all(setting.as_bytes())?;
        Ok(())
    }
}
