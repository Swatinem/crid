import Speck from "./speck";
import Base58 from "./base58";

/**
 * Crid - The cryptographic ID serializer
 *
 * It takes a `Number` and encodes it as a short-url in a way that:
 * * _appears_ random
 * * is hard to enumerate and predict
 *
 * Under the Hood, it uses the easy to implement Speck Cipher and
 * Base58 encoding.
 *
 * https://en.wikipedia.org/wiki/Speck_%28cipher%29
 * https://en.wikipedia.org/wiki/Base58
 */
class Crid {
  private speck: Speck;

  constructor(key: Uint32Array) {
    this.speck = new Speck(key);
  }

  public encode(hi: number, lo: number = 0) {
    const source = Uint32Array.of(hi, lo);
    const crypted = this.speck.encrypt(source);
    return Base58.encode(crypted);
  }
  public decode(str: string) {
    const decoded = Base58.decode(str);
    if (!decoded) return;
    return this.speck.decrypt(decoded);
  }
}

let neon = undefined;
try {
  neon = require('../native').Crid;
} catch {}

export {
  Crid as js,
  neon,
}

export default Crid;
