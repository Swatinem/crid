// @ts-ignore there are no @types yet :-(
import test from "zora";
import Speck from "../src/speck";
import rand from "./rand";

test("Speck", (t: any) => {
  t.test("should work for the test vectors", (t: any) => {

    let key = Uint32Array.of(0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100);
    let plaintext = Uint32Array.of(0x3b726574, 0x7475432d);
    let ciphertext = Uint32Array.of(0x8c6fa548, 0x454e028b);
    const speck = new Speck(key);
    const encoded = speck.encrypt(plaintext);
    t.equal(encoded[0], ciphertext[0], "speck encoded [0]");
    t.equal(encoded[1], ciphertext[1], "speck encoded [1]");
    const decoded = speck.decrypt(encoded)
    t.equal(decoded[0], plaintext[0], "speck decoded [0]");
    t.equal(decoded[1], plaintext[1], "speck decoded [1]");
  });

  t.test("should work for all randomized inputs", (t: any) => {
    try {
      for (let i = 0; i < 2 ** 10; i++) {
        const key = Uint32Array.of(rand(), rand(), rand(), rand());
        const speck = new Speck(key);
        for (let i = 0; i < 2 ** 10; i++) {
          const expected = Uint32Array.of(rand(), rand());
          const actual = speck.decrypt(speck.encrypt(expected));
          if (actual[0] !== expected[0] || actual[1] !== expected[1]) {
            throw new Error(`encrypting "${expected}" failed with key (${key})`);
          }
        }
      }
      t.ok(true, "speck randomized");
    } catch (e) {
      t.fail(e, "speck randomized");
    }
  });
});
