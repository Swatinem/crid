import { assertForall, utils } from "jsverify";
import Crid from "./";

let impls: any;
try {
  impls = require("./impls");
} catch (e) {
  console.error(e);
}

function testEachImpl(text: string, fn: (crid: typeof Crid) => void) {
  it(`${text} (js)`, () => fn(Crid));
  (impls ? it : it.skip)(`${text} (neon)`, () => fn(impls.neon));
  (impls ? it : it.skip)(`${text} (wasm)`, () => fn(impls.wasm));
}

describe("Crid", () => {
  testEachImpl("should work for specific input", Crid => {
    let key = [0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100];
    let plaintext = [0x3b726574, 0x7475432d];
    const crid = new Crid(key);
    const encoded = crid.encode(plaintext[0], plaintext[1]);
    expect(encoded).toEqual("QVQZbMUZ84r");
    const decoded = crid.decode(encoded);
    expect(decoded).toEqual(plaintext);
  });

  testEachImpl("should return undefined on invalid input", Crid => {
    let key = [0, 0, 0, 0];
    let crid = new Crid(key);
    expect(crid.decode("11111111110")).toBeUndefined();
    expect(crid.decode("1111111111")).toBeUndefined();
  });

  testEachImpl("should default second number to 0", Crid => {
    let key = [0, 0, 0, 0];
    let crid = new Crid(key);
    const encoded = crid.encode(1234);
    const decoded = crid.decode(encoded);
    expect(decoded).toEqual([1234, 0]);
  });

  testEachImpl("should quickcheck", Crid => {
    assertForall("uint32 & uint32 & uint32 & uint32", "uint32 & uint32", (key, expected) => {
      const crid = new Crid(key);
      const actual = crid.decode(crid.encode(expected[0], expected[1]));
      return utils.isEqual(actual, expected);
    });
  });
});
