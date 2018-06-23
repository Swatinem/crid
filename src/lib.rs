#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
mod test_base_x;
#[cfg(test)]
mod test_base58;
#[cfg(test)]
mod test_speck;
#[cfg(test)]
mod test_crid;

mod base58;
mod base_x;
mod speck;

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

  pub fn encode_str(&self, block: Block) -> String {
    let block = self.speck.encrypt(block);
    base58::encode_std(block)
  }

  pub fn decode(&self, string: &str) -> Option<Block> {
    let block = base58::decode(string)?;
    Some(self.speck.decrypt(block))
  }
}
