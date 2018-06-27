use super::base58::*;

quickcheck! {
  fn check_base_58(num: u64) -> bool {
    let mut buf = [0; CHARS];
    encode(num, &mut buf);
    let decoded = decode(&buf).unwrap();
    decoded == num
  }
}
