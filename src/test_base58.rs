use super::base58::*;

quickcheck! {
  fn check_base_58(hi: u32, lo: u32) -> bool {
    let source = [hi, lo];
    let encoded = encode_std(source);
    let decoded = decode(&encoded).unwrap();
    decoded == source
  }
}
