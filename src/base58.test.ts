import Base58 from "../src/base58";
import { assertForall, utils } from "jsverify";

describe("Base58", () => {
  it("should work for specific input", () => {
    const expected = [0, 0];
    let encoded = Base58.encode(expected);
    expect(encoded).toEqual("11111111111");
    const decoded = Base58.decode(encoded)!;
    expect(decoded).toEqual(expected);
  });

  it("should quickcheck", () => {
    assertForall("uint32 & uint32", expected => {
      const actual = Base58.decode(Base58.encode(expected))!;
      return utils.isEqual(actual, expected)
    });
  });
});
