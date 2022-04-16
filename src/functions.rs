use std::{fs::File, io::Write};

use neon::prelude::*;
use serde_json::Value;

use crate::{
    consts::{CACHE_DIR, CLIENT},
    emotes::{EmoteData, ParsedResult},
    helpers::array::emote_vec_to_array,
};

const SIZES: [u8; 3] = [1, 2, 3];

pub fn parse_string(mut cx: FunctionContext) -> JsResult<JsArray> {
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

pub fn get_bttv(mut cx: FunctionContext) -> JsResult<JsString> {
    if let Some(user_id_val) = cx.argument_opt(0) {
        let user_id = user_id_val
            .downcast::<JsString, FunctionContext>(&mut cx)
            .unwrap()
            .value(&mut cx);

        let cache_file_path = CACHE_DIR
            .clone()
            .unwrap()
            .join(format!("bttv.{}.json", user_id));

        let mut cache_file = File::create(cache_file_path).unwrap();
        let client = CLIENT.clone();

        let res = client
            .get(format!(
                "https://api.betterttv.net/3/cached/users/twitch/{}",
                user_id
            ))
            .send()
            .unwrap()
            .json::<Value>()
            .unwrap()
            .to_string();

        cache_file.write_all(res.as_bytes()).unwrap();

        Ok(cx.string(res))
    } else {
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
}
