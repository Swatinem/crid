// @ts-ignore there are no @types yet :-(
import test from "zora";
import Crid from "../src";
import rand48 from "./rand48";

import "./base58";
import "./speck";

test("Crid", (t: any) => {
  t.test("should work for specific input", (t: any) => {
    const crid = new Crid(0, 0);
    const encoded = crid.encode(0);
    t.equal(encoded, "26g2XNyV9", "crid encoded");
    t.equal(crid.decode(encoded), 0, "crid decoded");
  });

  t.test("should work for all randomized inputs", (t: any) => {
    try {
      for (let i = 0; i < 2 ** 10; i++) {
        const key1 = rand48();
        const key2 = rand48();
        const crid = new Crid(key1, key2);
        for (let i = 0; i < 2 ** 10; i++) {
          const num = rand48();
          const actual = crid.decode(crid.encode(num));
          if (num !== actual) {
            throw new Error(`encoding "${num}" failed with keys ("${key1}", "${key2}")`);
          }
        }
      }
      t.ok(true, "crid randomized");
    } catch (e) {
      t.fail(e, "crid randomized");
    }
  });
});
