use super::*;

#[test]
fn invalid_input() {
  let crid = Crid::new([0, 0, 0, 0]);
  assert_eq!(crid.decode("11111111110"), None);
  assert_eq!(crid.decode("1111111111"), None);
}

quickcheck! {
  fn check_crid(k1: u32, k2: u32, k3: u32, k4: u32, hi: u32, lo: u32) -> bool {
    let key = [k1, k2, k3, k4];
    let block = [hi, lo];
    let crid = Crid::new(key);

    let encoded = crid.encode_to_str(block);
    let decoded = crid.decode(&encoded).unwrap();
    decoded == block
  }
}
