pub const CHARS: usize = 11;
const BASE: u64 = 58;
pub type Buf = [u8; CHARS];
static ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
#[cfg_attr(rustfmt, rustfmt_skip)]
static REVERSE: &[i8; 128] = &[
  -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
  -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
  -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
  -1, 0, 1, 2, 3, 4, 5, 6,  7, 8,-1,-1,-1,-1,-1,-1,
  -1, 9,10,11,12,13,14,15, 16,-1,17,18,19,20,21,-1,
  22,23,24,25,26,27,28,29, 30,31,32,-1,-1,-1,-1,-1,
  -1,33,34,35,36,37,38,39, 40,41,42,43,-1,44,45,46,
  47,48,49,50,51,52,53,54, 55,56,57,-1,-1,-1,-1,-1,
];

pub fn encode(mut num: u64, string: &mut Buf) {
  for ch in string.iter_mut().rev() {
    let rem = num % BASE;
    num = num / BASE;
    *ch = unsafe { *ALPHABET.get_unchecked(rem as usize) };
  }
}

pub fn decode(string: &Buf) -> Option<u64> {
  let mut num = 0;

  for ch in string.iter() {
    let ch = *REVERSE.get(*ch as usize)?;
    if ch == -1 {
      return None;
    }
    num = num * BASE + ch as u64;
  }

  Some(num)
}
