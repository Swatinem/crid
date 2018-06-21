pub type Block = [u32; 2];
pub type Key = [u32; 4];

const ROUNDS: usize = 27;

fn round(x: &mut u32, y: &mut u32, k: u32) {
  *x = x.rotate_right(8);
  *x = x.wrapping_add(*y);
  *x ^= k;
  *y = y.rotate_left(3);
  *y ^= *x;
}

fn reverse(x: &mut u32, y: &mut u32, k: u32) {
  *y ^= *x;
  *y = y.rotate_right(3);
  *x ^= k;
  *x = x.wrapping_sub(*y);
  *x = x.rotate_left(8);
}

pub struct Speck {
  schedule: [u32; ROUNDS],
}

impl Speck {
  pub fn new(key: Key) -> Speck {
    let mut a = [key[0], key[1], key[2]];
    let mut b = key[3];
    let mut schedule = [0; ROUNDS];

    for i in 0..ROUNDS {
      schedule[i] = b;
      round(&mut a[2 - (i % 3)], &mut b, i as u32);
    }
    Speck { schedule }
  }

  pub fn encrypt(&self, block: Block) -> Block {
    let [mut x, mut y] = block;
    for &k in &self.schedule {
      round(&mut x, &mut y, k);
    }
    [x, y]
  }

  pub fn decrypt(&self, block: Block) -> Block {
    let [mut x, mut y] = block;
    for &k in self.schedule.iter().rev() {
      reverse(&mut x, &mut y, k);
    }
    [x, y]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_vectors() {
    let key = [0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100];
    let plaintext = [0x3b726574, 0x7475432d];
    let ciphertext = [0x8c6fa548, 0x454e028b];
    let speck = Speck::new(key);

    let encoded = speck.encrypt(plaintext);
    assert_eq!(encoded, ciphertext);
    let decoded = speck.decrypt(encoded);
    assert_eq!(decoded, plaintext);
  }

  quickcheck! {
    fn check_speck(nums: Vec<u32>) -> bool {
      if nums.len() < 6 { return true }
      let key = [nums[0], nums[1], nums[2], nums[3]];
      let block = [nums[4], nums[5]];
      let speck = Speck::new(key);

      let encoded = speck.encrypt(block);
      let decoded = speck.decrypt(encoded);
      decoded == block
    }
  }
}
