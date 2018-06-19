// @ts-ignore there are no @types yet :-(
import test from "zora";
import Speck from "../src/speck";
import rand48 from "./rand48";

test("Speck", (t: any) => {
  t.test("should work for specific input", (t: any) => {
    const speck = new Speck(0, 0);
    const encoded = speck.encrypt(0);
    t.equal(encoded, 140588681704008, "speck encoded");
    t.equal(speck.decrypt(encoded), 0, "speck decoded");
  });

  t.test("should work for all randomized inputs", (t: any) => {
    try {
      for (let i = 0; i < 2 ** 10; i++) {
        const key1 = rand48();
        const key2 = rand48();
        const speck = new Speck(key1, key2);
        for (let i = 0; i < 2 ** 10; i++) {
          const num = rand48();
          const actual = speck.decrypt(speck.encrypt(num));
          if (num !== actual) {
            throw new Error(`encoding "${num}" failed with keys ("${key1}", "${key2}")`);
          }
        }
      }
      t.ok(true, "speck randomized");
    } catch (e) {
      t.fail(e, "speck randomized");
    }
  });
});
