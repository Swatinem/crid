use super::speck::*;

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
  fn check_speck(k1: u32, k2: u32, k3: u32, k4: u32, hi: u32, lo: u32) -> bool {
    let key = [k1, k2, k3, k4];
    let block = [hi, lo];
    let speck = Speck::new(key);

    let encoded = speck.encrypt(block);
    let decoded = speck.decrypt(encoded);
    decoded == block
  }
}
