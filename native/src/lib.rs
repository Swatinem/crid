#[macro_use]
extern crate neon;

extern crate crid;

use crid::Crid;

use neon::js::class::Class;
use neon::js::Object;
use neon::js::{JsArray, JsNumber, JsObject, JsString, JsUndefined};
use neon::vm::Lock;

declare_types! {
  class JsCrid for Crid {
    init(call) {
      let scope = call.scope;
      let obj = call.arguments.require(scope, 0)?.check::<JsObject>()?;
      let key1 = obj.get(scope, 0)?.check::<JsNumber>()?.value() as u32;
      let key2 = obj.get(scope, 1)?.check::<JsNumber>()?.value() as u32;
      let key3 = obj.get(scope, 2)?.check::<JsNumber>()?.value() as u32;
      let key4 = obj.get(scope, 3)?.check::<JsNumber>()?.value() as u32;

      let key = [key1, key2, key3, key4];
      let crid = Crid::new(key);
      Ok(crid)
    }

    method encode(call) {
      let scope = call.scope;
      let hi = call.arguments.require(scope, 0)?.check::<JsNumber>()?.value() as u32;
      let lo = call.arguments.require(scope, 1)?.check::<JsNumber>()?.value() as u32;

      let res = call.arguments.this(scope).grab(|crid| {
        let block = [hi, lo];
        crid.encode_to_str(block)
      });

      Ok(JsString::new_or_throw(scope, &res)?.upcast())
    }

    method decode(call) {
      let scope = call.scope;
      let s = call.arguments.require(scope, 0)?.check::<JsString>()?.value();

      let res = call.arguments.this(scope).grab(|crid| {
        crid.decode(&s)
      });

      let res = match res {
        None => return Ok(JsUndefined::new().upcast()),
        Some(res) => res,
      };

      let array = JsArray::new(scope, 2);
      array.set(0, JsNumber::new(scope, res[0].into()))?;
      array.set(1, JsNumber::new(scope, res[1].into()))?;
      Ok(array.upcast())
    }
  }
}

register_module!(m, {
  m.exports
    .set("Crid", JsCrid::class(m.scope)?.constructor(m.scope)?)?;
  Ok(())
});
