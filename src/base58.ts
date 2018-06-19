const ALPHABET = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const BASE = ALPHABET.length;
const REVERSE: { [key: string]: number } = {};
for (let i = 0; i < BASE; i++) {
  const char = ALPHABET.charAt(i);
  REVERSE[char] = i;
}

/**
 * This is a simple base58 encoder
 *
 * Note that we only need to deal with 48bit numbers here.
 *
 * The code is mostly adapted from this JS implementation:
 * https://github.com/delight-im/ShortURL/blob/master/JavaScript/ShortURL.js
 */
class Base58 {
  public static encode(num: number) {
    let str = "";

    while (num > 0) {
      str = ALPHABET.charAt(num % BASE) + str;
      num = Math.floor(num / BASE);
    }
    return str;
  }

  public static decode(str: string) {
    let num = 0;
    for (const char of str) {
      num = num * BASE + REVERSE[char];
    }
    return num;
  }
}

export default Base58;
