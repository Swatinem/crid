import Crid from "./";
import { assertForall, utils } from "jsverify";

const IMPLS: Array<"js" | "wasm" | "neon"> = ["js", "wasm", "neon"];

function testEachImpl(text: string, fn: (crid: typeof Crid) => void) {
  for (const impl of IMPLS) {
    it(`${text} (${impl})`, () => {
      const Construct = function(key: Array<number>) {
        return Crid(key, impl);
      };
      return fn(Construct);
    });
  }
}

describe("Crid", () => {
  testEachImpl("should work for specific input", async Crid => {
    let key = [0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100];
    let plaintext = [0x3b726574, 0x7475432d];
    const crid = await Crid(key);
    const encoded = crid.encode(plaintext[0], plaintext[1]);
    expect(encoded).toEqual("QVQZbMUZ84r");
    const decoded = crid.decode(encoded);
    expect(decoded).toEqual(plaintext);
  });

  testEachImpl("should return undefined on invalid input", async Crid => {
    let key = [0, 0, 0, 0];
    let crid = await Crid(key);
    expect(crid.decode("11111111110")).toBeUndefined();
    expect(crid.decode("1111111111")).toBeUndefined();
  });

  testEachImpl("should quickcheck", async Crid => {
    return assertForall(
      "uint32 & uint32 & uint32 & uint32",
      "uint32 & uint32",
      async (key, expected) => {
        const crid = await Crid(key);
        const actual = crid.decode(crid.encode(expected[0], expected[1]));
        return utils.isEqual(actual, expected);
      },
    );
  });
});
