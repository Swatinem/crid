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

  pub fn encrypt(&self, num: u64) -> u64 {
    let mut x: u32 = (num >> 32) as u32;
    let mut y: u32 = num as u32;
    for &k in &self.schedule {
      round(&mut x, &mut y, k);
    }
    (x as u64) << 32 | y as u64
  }

  pub fn decrypt(&self, num: u64) -> u64 {
    let mut x: u32 = (num >> 32) as u32;
    let mut y: u32 = num as u32;
    for &k in self.schedule.iter().rev() {
      reverse(&mut x, &mut y, k);
    }
    (x as u64) << 32 | y as u64
  }
}
