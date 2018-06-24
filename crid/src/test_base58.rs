use super::base58::*;

quickcheck! {
  fn check_base_58(hi: u32, lo: u32) -> bool {
    let source = [hi, lo];
    let mut buf = [0; CHARS];
    encode(source, &mut buf);
    let decoded = decode(&buf).unwrap();
    decoded == source
  }
}
