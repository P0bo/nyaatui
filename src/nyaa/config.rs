use config::Config;
use log::error;
use std::collections::HashMap;
use std::path::PathBuf;

fn get_config_path() -> Option<PathBuf> {
    if let Some(mut x) = dirs::config_dir() {
        x.push("nyaa");
        x.push("config.toml");
        return Some(x.to_owned());
    }
    None
}

pub fn create_config() {
    if let Some(path) = get_config_path() {
        let parent = path.parent().unwrap();
        match std::fs::create_dir_all(parent) {
            Ok(_) => (),
            Err(_) => error!("Failed to create config folder"),
        }
        // match std::fs::File::create(path)
        let _ = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path.as_path().to_str().unwrap());
        let settings = Config::builder()
            .add_source(config::File::from(path.as_path()))
            .build()
            .unwrap();
        println!(
            "{:?}",
            settings
                .try_deserialize::<HashMap<String, HashMap<String, String>>>()
                .unwrap()
        )
    } else {
        error!("Failed to get config dir");
    }
}
