#![feature(proc_macro, wasm_import_module, wasm_custom_section)]

extern crate crid;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Crid(crid::Crid);

#[wasm_bindgen]
impl Crid {
  pub fn new(key: Vec<u32>) -> Crid {
    Crid(crid::Crid::new([0, 0, 0, 0]))
  }
  pub fn encode(&self, hi: u32, lo: u32) -> String {
    self.0.encode_to_str([hi, lo])
  }
  pub fn decode(&self, s: &str) -> Vec<u32> {
    match self.0.decode(s) {
      None => vec![0, 0, 0],
      Some(res) => {
        let mut v = vec![1];
        v.extend(&res);
        v
      }
    }
  }
}
