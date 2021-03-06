import Speck from "../src/speck";
import { assertForall, utils } from "jsverify";

describe("Speck", () => {
  it("should work for the test vectors", () => {
    let key = [0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100];
    let plaintext = [0x3b726574, 0x7475432d];
    let ciphertext = [0x8c6fa548, 0x454e028b];
    const speck = new Speck(key);
    const encoded = speck.encrypt(plaintext);
    expect(encoded).toEqual(ciphertext);
    const decoded = speck.decrypt(encoded);
    expect(decoded).toEqual(plaintext);
  });

  it("should quickcheck", () => {
    assertForall("uint32 & uint32 & uint32 & uint32", "uint32 & uint32", (key, expected) => {
      const speck = new Speck(key);
      const actual = speck.decrypt(speck.encrypt(expected));
      return utils.isEqual(actual, expected);
    });
  });
});
