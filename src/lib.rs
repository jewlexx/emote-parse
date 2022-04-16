use std::{fs::File, io::Write, path::PathBuf};

use directories::BaseDirs;
use neon::prelude::*;
use reqwest::blocking::ClientBuilder;
use serde_json::Value;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DIRS: Option<BaseDirs> = BaseDirs::new();
    static ref CACHE_DIR: Option<PathBuf> = DIRS.as_ref().map(|dirs| {
        let path = dirs.cache_dir().to_owned().join("ttvparse");

        if !path.exists() {
            std::fs::create_dir_all(&path).unwrap();
        }

        path
    });
    static ref CLIENT: reqwest::blocking::Client = {
        let builder = ClientBuilder::new();
        builder.build().unwrap()
    };
}

fn get_bttv(mut cx: FunctionContext) -> JsResult<JsString> {
    let cache_file_path = CACHE_DIR.clone().unwrap().join("bttv.json");
    // let meta = cache_file.metadata().unwrap();
    // let last_modified = meta.modified().unwrap();

    let mut cache_file = File::create(cache_file_path).unwrap();

    let client = CLIENT.clone();

    let res = client
        .get("https://api.betterttv.net/3/cached/emotes/global")
        .send()
        .unwrap()
        .json::<Value>()
        .unwrap()
        .to_string();

    cache_file.write_all(res.as_bytes()).unwrap();

    Ok(cx.string(res))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getBTTV", get_bttv)?;
    Ok(())
}
