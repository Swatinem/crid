// https://github.com/bitcoin/bitcoin/blob/master/src/base58.cpp
use base_x::convert32;
use super::Block;

const CHARS: usize = 11;
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

pub fn encode(block: Block) -> [u8; 11] {
  let mut buf = [0; CHARS];

  convert32(&block, 1 << 32, &mut buf, 58);

  let mut string = [0; CHARS];
  for (ch, bufch) in string.iter_mut().zip(buf.iter()) {
    *ch = ALPHABET[*bufch as usize];
  }
  string
}

pub fn encode_std(block: Block) -> String {
  let string = encode(block);
  unsafe { String::from_utf8_unchecked(string.to_vec()) }
}

pub fn decode(string: &str) -> Option<Block> {
  if string.len() < CHARS {
    return None;
  }
  let mut buf = [0; CHARS];

  for (ch, bufch) in string
    .chars()
    .zip(buf.iter_mut())
  {
    let ch = *REVERSE.get(ch as usize)?;
    if ch == -1 {
      return None;
    }
    *bufch = ch as u32
  }

  let mut block = [0; 2];
  convert32(&buf, 58, &mut block, 1 << 32);

  Some(block)
}

#[cfg(test)]
mod tests {
  use super::*;

  quickcheck! {
    fn check_base_58(hi: u32, lo: u32) -> bool {
      let source = [hi, lo];
      let encoded = encode_std(source);
      let decoded = decode(&encoded).unwrap();
      decoded == source
    }
  }
}
