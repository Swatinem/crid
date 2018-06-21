// @ts-ignore there are no @types yet :-(
import test from "zora";
import Base58 from "../src/base58";
import rand from "./rand";

test("Base58", (t: any) => {
  t.test("should work for specific input", (t: any) => {
    let encoded = Base58.encode(Uint32Array.of(0, 0));
    t.equal(encoded, "11111111111", "base58 encoded 1");
    const decoded = Base58.decode(encoded);
    t.equal(decoded[0], 0, "base58 decoded [0]");
    t.equal(decoded[1], 0, "base58 decoded [1]");
  });

  t.test("should work for all randomized inputs", (t: any) => {
    try {
      for (let i = 0; i < 2 ** 20; i++) {
        const expected = Uint32Array.of(rand(), rand());
        const actual = Base58.decode(Base58.encode(expected));
        if (actual[0] !== expected[0] || actual[1] !== expected[1]) {
          throw new Error(`Base58 encoding "${expected}" failed`);
        }
      }
      t.ok(true, "base58 randomized");
    } catch (e) {
      t.fail(e, "base58 randomized");
    }
  });
});
