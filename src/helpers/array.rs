use neon::prelude::*;

use crate::emotes::ParsedResult;

pub fn array_to_array<'a, C: Context<'a>>(array: &[String], cx: &mut C) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, array.len() as u32);

    for (i, s) in array.iter().enumerate() {
        let v = cx.string(s);
        a.set(cx, i as u32, v)?;
    }

    Ok(a)
}

pub fn emote_vec_to_array<'a, C: Context<'a>>(
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
