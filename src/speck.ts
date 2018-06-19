const ROUNDS = 23;
const BITS = 24;
const DIV = 1 << BITS;
const MASK = DIV - 1;

/**
 * This is the Speck Cipher, explained here:
 * https://eprint.iacr.org/2013/404.pdf
 *
 * We use the 48/96 Variant.
 * Since in JS, `Number.MAX_SAFE_INTEGER` is 53bit, we can encode
 * one 48bit block as one number, and the 96bit key as two numbers.
 *
 * The code is mostly adapted from this JS implementation:
 * https://github.com/hax/speck.js/blob/master/lib/48/96.js
 */
class Speck {
  private key = new Uint32Array(ROUNDS);

  constructor(key1: number, key2: number) {
    let a = Uint32Array.of(key1 & MASK, (key1 / DIV) & MASK, key2 & MASK);
    let b = (key2 / DIV) & MASK;
    for (let i = 0; i < ROUNDS; i++) {
      this.key[i] = b;
      const j = 2 - (i % 3);
      a[j] = ((((a[j] << 16) | (a[j] >>> 8)) + b) & MASK) ^ i;
      b = (((b << 3) | (b >>> 21)) & MASK) ^ a[j];
    }
  }

  public encrypt(num: number) {
    let x = num & MASK;
    let y = (num / DIV) & MASK;

    for (let i = 0; i < ROUNDS; i++) {
      x = ((((x << 16) | (x >>> 8)) + y) & MASK) ^ this.key[i];
      y = (((y << 3) | (y >>> 21)) & MASK) ^ x;
    }

    num = x + y * DIV;
    return num;
  }

  public decrypt(num: number) {
    let x = num & MASK;
    let y = (num / DIV) & MASK;

    for (let i = ROUNDS - 1; i >= 0; i--) {
      y = x ^ y;
      y = ((y << 21) | (y >>> 3)) & MASK;
      x = ((x ^ this.key[i]) - y) & MASK;
      x = ((x << 8) | (x >>> 16)) & MASK;
    }

    num = x + y * DIV;
    return num;
  }
}

export default Speck;
