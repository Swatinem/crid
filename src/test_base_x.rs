use super::base_x::*;

fn dest_length(source_len: usize, source_base: usize, dest_base: usize) -> usize {
  (source_len as f64 * (source_base as f64).log(dest_base as f64)).trunc() as usize + 1
}

quickcheck! {
  fn check_conversion_32(source_base: usize, dest_base: usize, source: Vec<u32>) -> bool {
    let base_valid =
      source_base > 2 && source_base <= 1 << 32 &&
      dest_base > 2 && dest_base <= 1 << 32;
    let source_valid = !source.is_empty() &&
      !source.iter().any(|el| (*el as usize) >= source_base);
    if !base_valid || !source_valid {
      return true;
    }
    let source_base = source_base as usize;
    let dest_base = dest_base as usize;

    let dest_len = dest_length(source.len(), source_base, dest_base);
    let mut dest = vec![0; dest_len];
    convert32(&source, source_base, &mut dest, dest_base);

    let mut actual = vec![0; source.len()];
    convert32(&dest, dest_base, &mut actual, source_base);

    source == actual
  }
}
