// @ts-ignore there are no @types yet :-(
import test from "zora";
import Crid from "../src";
import rand from "./rand";

import "./base58";
import "./speck";

test("Crid", (t: any) => {
  t.test("should work for specific input", (t: any) => {
    let key = Uint32Array.of(0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100);
    let plaintext = Uint32Array.of(0x3b726574, 0x7475432d);
    const crid = new Crid(key);
    const encoded = crid.encode(plaintext[0], plaintext[1]);
    t.equal(encoded, "QVQZbMUZ84r", "crid encoded");
    const decoded = crid.decode(encoded);
    t.equal(decoded[0], plaintext[0], "crid decoded [0]");
    t.equal(decoded[1], plaintext[1], "crid decoded [1]");
  });

  t.test("should return undefined on invalid input", (t: any) => {
    let key = Uint32Array.of(0,0,0,0);
    let crid = new Crid(key);
    t.equal(crid.decode("11111111110"), undefined, "invalid chars");
    t.equal(crid.decode("1111111111"), undefined, "wrong length");
  });

  t.test("should work for all randomized inputs", (t: any) => {
    try {
      for (let i = 0; i < 2 ** 10; i++) {
        const key = Uint32Array.of(rand(), rand(), rand(), rand());
        const crid = new Crid(key);
        for (let i = 0; i < 2 ** 10; i++) {
          const num1 = rand();
          const num2 = rand();
          const actual = crid.decode(crid.encode(num1, num2));
          if (num1 !== actual[0] || num2 !== actual[1]) {
            throw new Error(`encoding "${num1}/${num2}" failed with key (${key})`);
          }
        }
      }
      t.ok(true, "crid randomized");
    } catch (e) {
      t.fail(e, "crid randomized");
    }
  });
});
