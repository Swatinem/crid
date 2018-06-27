#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
mod test_base58;
#[cfg(test)]
mod test_crid;
#[cfg(test)]
mod test_speck;

mod base58;
mod speck;

pub use base58::Buf;
use base58::CHARS;
pub use speck::Key;
use speck::Speck;

pub struct Crid {
  speck: Speck,
}

impl Crid {
  pub fn new(key: Key) -> Crid {
    Crid {
      speck: Speck::new(key),
    }
  }

  pub fn encode_into_buf(&self, num: u64, buf: &mut Buf) {
    let num = self.speck.encrypt(num);
    base58::encode(num, buf);
  }

  pub fn encode_to_str(&self, num: u64) -> String {
    let mut string = [0; CHARS];
    self.encode_into_buf(num, &mut string);
    unsafe { String::from_utf8_unchecked(string.to_vec()) }
  }

  pub fn decode(&self, string: &str) -> Option<u64> {
    if string.len() < CHARS {
      return None;
    }
    let buf = unsafe { std::mem::transmute(string.as_bytes().as_ptr()) };
    let num = base58::decode(buf)?;
    Some(self.speck.decrypt(num))
  }
}
