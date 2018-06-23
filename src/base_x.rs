/// This is a low-level base converter.
/// Code is adopted mostly from https://github.com/bitcoin/bitcoin/blob/master/src/base58.cpp
/// and changed to accept arbitrary bases
pub fn convert32(source: &[u32], source_base: usize, dest: &mut [u32], dest_base: usize) {
  let mut length = dest.len();
  for digit in source.iter() {
    let mut carry: usize = *digit as usize;
    let mut wrote_length = 0;
    for dest_digit in dest.iter_mut().rev() {
      if carry == 0 && wrote_length >= length {
        break;
      }
      carry += source_base * *dest_digit as usize;
      *dest_digit = (carry % dest_base) as u32;
      carry /= dest_base;
      wrote_length += 1;
    }
    length = wrote_length;
  }
}
