// @ts-ignore there are no @types yet :-(
import test from "zora";
import Base58 from "../src/base58";
import rand48 from "./rand48";

test("Base58", (t: any) => {
  t.test("should work for specific input", (t: any) => {
    let encoded = Base58.encode(1);
    t.equal(encoded, "2", "base58 encoded 1");
    t.equal(Base58.decode(encoded), 1, "base58 decoded 1");
  });

  t.test("should work for all randomized inputs", (t: any) => {
    try {
      for (let i = 0; i < 2 ** 20; i++) {
        const num = rand48();
        const actual = Base58.decode(Base58.encode(num));
        if (num !== actual) {
          throw new Error(`Base58 encoding "${num}" failed`);
        }
      }
      t.ok(true, "base58 randomized");
    } catch (e) {
      t.fail(e, "base58 randomized");
    }
  });
});
