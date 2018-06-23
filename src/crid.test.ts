import Crid from "./";
import { assertForall, utils } from "jsverify";

describe("Crid", () => {
  it("should work for specific input", () => {
    let key = Uint32Array.of(0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100);
    let plaintext = Uint32Array.of(0x3b726574, 0x7475432d);
    const crid = new Crid(key);
    const encoded = crid.encode(plaintext[0], plaintext[1]);
    expect(encoded).toEqual("QVQZbMUZ84r");
    const decoded = crid.decode(encoded);
    expect(decoded).toEqual(plaintext);
  });

  it("should return undefined on invalid input", () => {
    let key = Uint32Array.of(0, 0, 0, 0);
    let crid = new Crid(key);
    expect(crid.decode("11111111110")).toBeUndefined();
    expect(crid.decode("1111111111")).toBeUndefined();
  });

  it("should quickcheck", () => {
    assertForall("uint32 & uint32 & uint32 & uint32", "uint32 & uint32", (_key, num) => {
      const key = Uint32Array.from(_key);
      const crid = new Crid(key);
      const expected = Uint32Array.from(num);
      const actual = crid.decode(crid.encode(num[0], num[1]));
      return utils.isEqual(actual, expected);
    });
  });
});
