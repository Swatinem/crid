#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub mod base58;
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn invalid_input() {
    let crid = Crid::new([0,0,0,0]);
    assert_eq!(crid.decode("11111111110"), None);
    assert_eq!(crid.decode("1111111111"), None);
  }

  quickcheck! {
    fn check_crid(nums: Vec<u32>) -> bool {
      if nums.len() < 6 { return true }
      let key = [nums[0], nums[1], nums[2], nums[3]];
      let block = [nums[4], nums[5]];
      let crid = Crid::new(key);

      let encoded = crid.encode_str(block);
      let decoded = crid.decode(&encoded).unwrap();
      decoded == block
    }
  }
}
