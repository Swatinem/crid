# Crid - The cryptographic ID serializer

[![Build Status](https://img.shields.io/travis/Swatinem/crid.svg)](https://travis-ci.org/Swatinem/crid)
[![Coverage Status](https://img.shields.io/codecov/c/github/Swatinem/crid.svg)](https://codecov.io/gh/Swatinem/crid)

It takes one or two `Number`s and encodes them as a 11 char short-url in a way that:

- _appears_ random
- is hard to enumerate and predict

It is also especially built to be **tiny**, with:

- **zero dependencies**
- **~130 source-lines-of-code**

Under the Hood, it uses the easy to implement [Speck Cipher][speck] and
[Base58][base58] encoding.

[speck]: https://en.wikipedia.org/wiki/Speck_%28cipher%29
[base58]: https://en.wikipedia.org/wiki/Base58

Apart from that, it is a testbed for my personal [rust] experiments :-)

[rust]: https://rust-lang.org/
