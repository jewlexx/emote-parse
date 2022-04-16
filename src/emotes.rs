use neon::prelude::*;

use crate::helpers::array::array_to_array;

#[derive(Debug, Clone)]
pub struct EmoteData {
    pub id: String,
    pub code: String,
    pub image_type: String,
    pub user_id: String,
}

impl EmoteData {
    pub fn from_object<'a, C: Context<'a>>(cx: &mut C, obj: Handle<JsObject>) -> Self {
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

impl EmoteData {
    pub fn to_object<'a, C: Context<'a>>(&self, cx: &mut C) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let id = cx.string(&self.id);
        obj.set(cx, "id", id)?;

        let code = cx.string(&self.code);
        obj.set(cx, "code", code)?;

        let image_type = cx.string(&self.image_type);
        obj.set(cx, "imageType", image_type)?;

        let user_id = cx.string(&self.user_id);
        obj.set(cx, "userId", user_id)?;

        Ok(obj)
    }
}

pub struct ParsedResult {
    pub emote: EmoteData,
    pub index: i32,
    pub end_index: i32,
    pub urls: [String; 3],
}

impl ParsedResult {
    pub fn to_object<'a, C: Context<'a>>(&self, cx: &mut C) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let emote = self.emote.to_object(cx)?;
        obj.set(cx, "emote", emote)?;

        let index = cx.number(self.index);
        obj.set(cx, "index", index)?;

        let end_index = cx.number(self.end_index);
        obj.set(cx, "endIndex", end_index)?;

        let urls = array_to_array(&self.urls, cx)?;
        obj.set(cx, "urls", urls)?;

        Ok(obj)
    }
}
