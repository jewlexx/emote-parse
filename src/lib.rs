use std::{fs::File, io::Write, path::PathBuf};

use directories::BaseDirs;
use neon::prelude::*;
use reqwest::blocking::ClientBuilder;
use serde_json::Value;

mod emotes;
use emotes::{EmoteData, ParsedResult};

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

fn array_to_array<'a, C: Context<'a>>(array: &[String], cx: &mut C) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, array.len() as u32);

    for (i, s) in array.iter().enumerate() {
        let v = cx.string(s);
        a.set(cx, i as u32, v)?;
    }

    Ok(a)
}

fn emote_vec_to_array<'a, C: Context<'a>>(
    vec: Vec<ParsedResult>,
    cx: &mut C,
) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, vec.len() as u32);

    for (i, s) in vec.iter().enumerate() {
        let v = s.to_object(cx)?;
        a.set(cx, i as u32, v)?;
    }

    Ok(a)
}

const SIZES: [u8; 3] = [1, 2, 3];

fn parse_string(mut cx: FunctionContext) -> JsResult<JsArray> {
    let string = cx.argument::<JsString>(0)?.value(&mut cx);
    let emotes = cx.argument::<JsArray>(1)?;

    let emotes_vec = emotes.to_vec(&mut cx)?;

    let mut parsed: Vec<ParsedResult> = Vec::new();

    for emote in emotes_vec {
        let emote_obj = emote
            .downcast::<JsObject, FunctionContext>(&mut cx)
            .unwrap();

        let casted_emote = EmoteData::from_object(&mut cx, emote_obj);

        let urls = SIZES.map(|size| {
            format!(
                "https://cdn.betterttv.net/emote/{}/{}x",
                casted_emote.id, size
            )
        });

        let mut indexes = Vec::<(usize, usize)>::new();

        let mut start: usize = 0;
        loop {
            let sliced_string = &string[start..];
            let index = sliced_string.find(&casted_emote.code);

            match index {
                Some(i) => {
                    indexes.push((i, i + casted_emote.code.len()));
                    start += i + 1;
                }
                None => break,
            }
        }

        for (index, end_index) in indexes {
            parsed.push(ParsedResult {
                emote: casted_emote.clone(),
                index: index as i32,
                end_index: end_index as i32,
                urls: urls.clone(),
            });
        }
    }

    emote_vec_to_array(parsed, &mut cx)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getBTTV", get_bttv)?;
    cx.export_function("parseString", parse_string)?;
    Ok(())
}
