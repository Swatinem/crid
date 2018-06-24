#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
mod test_base58;
#[cfg(test)]
mod test_base_x;
#[cfg(test)]
mod test_crid;
#[cfg(test)]
mod test_speck;

mod base58;
mod base_x;
mod speck;

pub use base58::Buf;
use base58::CHARS;
use speck::Speck;
pub use speck::{Block, Key};

pub struct Crid {
  speck: Speck,
}

impl Crid {
  pub fn new(key: Key) -> Crid {
    Crid {
      speck: Speck::new(key),
    }
  }

  pub fn encode_into_buf(&self, block: Block, buf: &mut Buf) {
    let block = self.speck.encrypt(block);
    base58::encode(block, buf);
  }

  pub fn encode_to_str(&self, block: Block) -> String {
    let mut string = [0; CHARS];
    self.encode_into_buf(block, &mut string);
    unsafe { String::from_utf8_unchecked(string.to_vec()) }
  }

  pub fn decode(&self, string: &str) -> Option<Block> {
    if string.len() < CHARS {
      return None;
    }
    let buf = unsafe { std::mem::transmute(string.as_bytes().as_ptr()) };
    let block = base58::decode(buf)?;
    Some(self.speck.decrypt(block))
  }
}

// #[macro_use]
// extern crate neon;

// use neon::js::class::Class;
// use neon::js::{JsFunction, JsNumber, JsObject, JsString, JsUndefined};
// use neon::js::{Object};
// use neon::scope::Scope;
// use neon::vm::{Lock};

// declare_types! {
//   class JsCrid for Crid {
//     init(call) {
//       let scope = call.scope;
//       let obj = call.arguments.require(scope, 0)?.check::<JsObject>()?;
//       let key1 = obj.get(scope, 0)?.check::<JsNumber>()?.value() as u32;
//       let key2 = obj.get(scope, 1)?.check::<JsNumber>()?.value() as u32;
//       let key3 = obj.get(scope, 2)?.check::<JsNumber>()?.value() as u32;
//       let key4 = obj.get(scope, 3)?.check::<JsNumber>()?.value() as u32;

//       let key = [key1, key2, key3, key4];
//       let crid = Crid::new(key);
//       Ok(crid)
//     }

//     method encode(call) {
//       let scope = call.scope;
//       let hi = call.arguments.require(scope, 0)?.check::<JsNumber>()?.value() as u32;
//       let lo = call.arguments.require(scope, 1)?.check::<JsNumber>()?.value() as u32;

//       let res = call.arguments.this(scope).grab(|crid| {
//         let block = [hi, lo];
//         crid.encode_to_str(block)
//       });

//       Ok(JsString::new_or_throw(scope, &res)?.upcast())
//     }

//     method decode(call) {
//       let scope = call.scope;
//       let s = call.arguments.require(scope, 0)?.check::<JsString>()?.value();

//       let res = call.arguments.this(scope).grab(|crid| {
//         crid.decode(&s)
//       });

//       let res = match res {
//         None => return Ok(JsUndefined::new().upcast()),
//         Some(res) => res,
//       };

//       let uint32_array = scope.global().get(scope, "Uint32Array")?.check::<JsObject>()?;
//       let of = uint32_array.get(scope, "of")?.check::<JsFunction>()?;
//       let items = [
//         JsNumber::new(scope, res[0].into()),
//         JsNumber::new(scope, res[1].into()),
//       ];
//       let res = of.call(scope, uint32_array, items.into_iter().cloned())?;
//       Ok(res.upcast())
//     }
//   }
// }

// register_module!(m, {
//   m.exports
//     .set("Crid", JsCrid::class(m.scope)?.constructor(m.scope)?)?;
//   Ok(())
// });
