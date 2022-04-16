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
    let cache_file_path = CACHE_DIR.clone().unwrap().join("bttv.global.json");
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

struct EmoteData {
    pub id: String,
    pub code: String,
    pub image_type: String,
    pub user_id: String,
}

impl EmoteData {
    fn from_object<'a, C>(cx: &mut C, obj: Handle<JsObject>) -> Self
    where
        C: Context<'a>,
    {
        let id = obj.get::<JsString, C, &str>(cx, "id").unwrap().value(cx);
        let code = obj.get::<JsString, C, &str>(cx, "code").unwrap().value(cx);
        let image_type = obj
            .get::<JsString, C, &str>(cx, "imageType")
            .unwrap()
            .value(cx);
        let user_id = obj
            .get::<JsString, C, &str>(cx, "userId")
            .unwrap()
            .value(cx);

        EmoteData {
            id,
            code,
            image_type,
            user_id,
        }
    }
}

struct ParsedResult {
    pub emote: EmoteData,
    pub index: i32,
    pub urls: [String; 3],
}

fn vec_to_array<'a, C: Context<'a>>(vec: Vec<String>, cx: &mut C) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, vec.len() as u32);

    for (i, s) in vec.iter().enumerate() {
        let v = cx.string(s);
        a.set(cx, i as u32, v)?;
    }

    Ok(a)
}

const SIZES: [u8; 3] = [1, 2, 3];

fn parse_string(mut cx: FunctionContext) -> JsResult<JsArray> {
    let string = cx.argument::<JsString>(0)?;
    let emotes = cx.argument::<JsArray>(1)?;

    let emotes_vec = emotes.to_vec(&mut cx)?;

    let mut parsed: Vec<ParsedResult> = Vec::new();

    for (index, emote) in emotes_vec.iter().enumerate() {
        let emote_obj = emote
            .downcast::<JsObject, FunctionContext>(&mut cx)
            .unwrap();

        let casted_emote = EmoteData::from_object(&mut cx, emote_obj);

        let urls = SIZES.clone().map(|size| {
            format!(
                "https://cdn.betterttv.net/emote/{}/{}x",
                casted_emote.id, size
            )
        });

        let result = ParsedResult {
            emote: casted_emote,
            index: index as i32,
            urls,
        };
    }

    vec_to_array(parsed, &mut cx)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getBTTV", get_bttv)?;
    Ok(())
}
