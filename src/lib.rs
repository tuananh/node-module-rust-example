extern crate napi;
#[macro_use]
extern crate napi_derive;

#[cfg(target_os = "macos")]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use napi::{CallContext, JsObject, JsString, Result};

#[js_function(3)]
fn say_hello(ctx: CallContext) -> Result<JsString> {
  let name = ctx.get::<JsString>(0)?.into_utf8()?;
  let name = name.as_str()?;
  let s = ctx.env.create_string_from_std(format!("Hello, {}!", name))?;
  Ok(s)
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("say_hello", say_hello)?;

  Ok(())
}
