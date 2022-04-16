use std::path::PathBuf;

use directories::BaseDirs;
use reqwest::blocking::ClientBuilder;

lazy_static! {
    static ref DIRS: Option<BaseDirs> = BaseDirs::new();
    pub static ref CACHE_DIR: Option<PathBuf> = DIRS.as_ref().map(|dirs| {
        let path = dirs.cache_dir().to_owned().join("ttvparse");

        if !path.exists() {
            std::fs::create_dir_all(&path).unwrap();
        }

        path
    });
    pub static ref CLIENT: reqwest::blocking::Client = {
        let builder = ClientBuilder::new();
        builder.build().unwrap()
    };
}
