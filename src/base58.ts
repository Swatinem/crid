import { convert32 } from "./base-x";

const CHARS = 11;
const U32 = 0xffffffff + 1;
const ALPHABET = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const BASE = ALPHABET.length;
const REVERSE: { [key: string]: number } = {};
for (let i = 0; i < BASE; i++) {
  const char = ALPHABET.charAt(i);
  REVERSE[char] = i;
}

class Base58 {
  private static buf = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

  public static encode(source: Array<number>) {
    const { buf } = this;
    buf.fill(0);
    convert32(source, U32, buf, 58);
    let str = "";
    for (const char of buf) {
      str += ALPHABET.charAt(char % BASE);
    }
    return str;
  }

  public static decode(str: string) {
    const { buf } = this;
    for (let i = 0; i < CHARS; i++) {
      const char = REVERSE[str[i]];
      if (char === undefined) {
        return undefined;
      }
      buf[i] = char;
    }
    const out = [0, 0];
    convert32(buf, 58, out, U32);
    return out;
  }
}

export default Base58;
