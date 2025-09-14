use std::{collections::HashMap, fs};

use crate::{downloader::{Downloader, DownloaderConfig}, module::Module, unzip::unzip, ANTMAN_PATH, CONFIG};

pub(crate) async fn add_package_handler(package_name: String) -> Result<(), String> {
    let modules_path = ANTMAN_PATH.clone().join("modules");

    if !modules_path.exists() {
        panic!("{} not exists", modules_path.to_str().unwrap())
    }

    if !modules_path.is_dir() {
        panic!("{} not a folder", modules_path.to_str().unwrap())
    }

    let url = {
        let client = reqwest::Client::new();
    
        // 获取索引文件内容
        let response = match client.get(&CONFIG.mod_index)
            .send()
            .await 
        {
            Ok(result) => result,
            Err(err) => return Err(err.to_string())
        };

        let content = match response.text().await {
            Ok(it) => it,
            Err(err) => return Err(err.to_string())
        };

        match serde_json::from_str::<HashMap<String, Module>>(&content) {
            Ok(m) => match m.get(&package_name) {
                Some(it) => &it.url.clone(),
                None => return Err(format!("cannot found module: {package_name}"))
            },
            Err(err) => return Err(err.to_string())
        }
    };

    let config = DownloaderConfig {
        max_concurrent_downloads: 12,
        user_agent: Some(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36 Edg/140.0.0.0".to_string()
        ),
        ..Default::default()
    };

    let downloader = match Downloader::new(config) {
        Ok(it) => it,
        Err(err) => return Err(err.to_string()),
    };

    let ziped_module = match 
        downloader.download_file(
            &url, modules_path.join(format!("{package_name}.zip"))
        ).await 
    {
        Ok(it) => it,
        Err(err) => return Err(err.to_string())
    };

    match unzip(
        &ziped_module, &modules_path.join(package_name)
    ) {
        Ok(it) => it,
        Err(err) => return Err(err.to_string())
    }

    match fs::remove_file(ziped_module) {
        Ok(_) => Ok(()),
        Err(err) => return Err(err.to_string())
    }
}