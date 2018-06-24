const ROUNDS = 27;

/**
 * This is the Speck Cipher, explained here:
 * https://eprint.iacr.org/2013/404.pdf
 *
 * We use the 64/128 Variant.
 *
 * The code is mostly adapted from this JS implementation:
 * https://github.com/hax/speck.js/blob/master/lib/64/128.js
 */
class Speck {
  private schedule = new Uint32Array(ROUNDS);

  constructor(key: Array<number>) {
    let a = Uint32Array.from(key);
    let b = key[3];

    for (let i = 0; i < ROUNDS; i++) {
      this.schedule[i] = b;
      const j = 2 - (i % 3);
      a[j] = (((a[j] << 24) | (a[j] >>> 8)) + b) ^ i;
      b = ((b << 3) | (b >>> 29)) ^ a[j];
    }
  }

  public encrypt(block: Array<number>) {
    let x = block[0];
    let y = block[1];
    const { schedule } = this;

    for (let i = 0; i < ROUNDS; i++) {
      x = (((x << 24) | (x >>> 8)) + y) ^ schedule[i];
      y = ((y << 3) | (y >>> 29)) ^ x;
    }

    return [x >>> 0, y >>> 0];
  }

  public decrypt(block: Array<number>) {
    let x = block[0];
    let y = block[1];
    const { schedule } = this;

    for (let i = ROUNDS - 1; i >= 0; i--) {
      y = x ^ y;
      y = (y << 29) | (y >>> 3);
      x = (x ^ schedule[i]) - y;
      x = (x << 8) | (x >>> 24);
    }

    return [x >>> 0, y >>> 0];
  }
}

export default Speck;
