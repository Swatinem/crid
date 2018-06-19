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

  constructor(key1: number, key2: number) {
    this.speck = new Speck(key1, key2);
  }

  public encode(num: number) {
    const crypted = this.speck.encrypt(num);
    return Base58.encode(crypted);
  }
  public decode(str: string) {
    const decoded = Base58.decode(str);
    return this.speck.decrypt(decoded);
  }
}

export default Crid;
