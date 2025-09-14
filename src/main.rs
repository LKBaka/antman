mod args;
mod handler;
mod downloader;
mod config;
mod unzip;
mod module;

use std::{env::home_dir, fs, io, path::{Path, PathBuf}};

use clap::Parser;
use once_cell::sync::Lazy;

use crate::{config::Config, handler::add_package_handler::add_package_handler};

const NORMAL_CONFIG: &str = r#"{
    "mod_index": "https://raw.githubusercontent.com/LKBaka/AntMods/main/mods.json"
}"#;

fn init() -> Result<(), io::Error> {
    let mut antman_path = home_dir().ok_or_else(
        || io::Error::new(io::ErrorKind::NotFound, "cannot found home path")
    )?;

    antman_path.push(".antman");

    fs::create_dir_all(&antman_path)?;

    global_env::set_global_env(
        "ANTMAN_PATH",
        antman_path.to_str().unwrap()
    )?;

    let mut config_path = antman_path.clone();
    config_path.push("config.json");

    fs::write(
        config_path,
        NORMAL_CONFIG
    )?;

    let mut mod_path = antman_path.clone();
    mod_path.push("modules");

    fs::create_dir_all(mod_path)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let args = args::Args::parse();

    if args.init {
        return init()
    }

    if let Some(it) = args.add {
        let result = add_package_handler(it).await;
        if let Err(err) = result {
            return Err(io::Error::new(io::ErrorKind::Other, err))
        }
    }

    Ok(())
}

pub static ANTMAN_PATH: Lazy<PathBuf> = Lazy::new(
    || {
        let v = global_env::get_global_env("ANTMAN_PATH");
        let antman_path = match &v {
            Some(it) => Path::new(it),
            None => {
                panic!(
                    "{:?}",
                    io::Error::new(io::ErrorKind::NotFound, "cannot found env var ANTMAN_PATH")
                )
            }
        };

        if !antman_path.exists() {
            panic!(
                "{:?}",
                io::Error::new(io::ErrorKind::NotFound, format!("cannot found path: {antman_path:?}"))
            )
        }
        
        antman_path.to_owned()
    }
);

pub static CONFIG: Lazy<Config> = Lazy::new(
    || serde_json::from_str::<Config>(
        &fs::read_to_string(ANTMAN_PATH.clone().join("config.json")).expect(
            &format!(
                "{:?}",
                io::Error::new(io::ErrorKind::NotFound, format!("cannot found path: {ANTMAN_PATH:?}"))
            )
        )
    ).map_err(
        |e| panic!(
            "{:?}",
            io::Error::new(io::ErrorKind::Unsupported, format!("deserialize failed: {e}"))
        )
    ).unwrap()
);