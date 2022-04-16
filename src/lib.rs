mod consts;
mod emotes;
mod functions;
mod helpers;

use neon::prelude::*;

use crate::functions::*;

#[macro_use]
extern crate lazy_static;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getBTTV", get_bttv)?;
    cx.export_function("parseString", parse_string)?;
    Ok(())
}
