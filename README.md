# Crid - The cryptographic ID serializer

[![Build Status](hhttps://img.shields.io/travis/Swatinem/crid.svg)](https://travis-ci.org/Swatinem/crid)
[![Coverage Status](https://img.shields.io/codecov/c/github/Swatinem/crid.svg)](https://codecov.io/gh/Swatinem/crid)

It takes a `Number` and encodes it as a short-url in a way that:

- _appears_ random
- is hard to enumerate and predict

It is also especially built to be **tiny**, with:

- **zero dependencies**
- **<100 source-lines-of-code**

To find the right balance between encoded ID length and address space, it uses
48bit Numbers, which are encoded as Strings up to 9 chars long.
That is up to `281_474_976_710_656` different IDs, which should be plenty,
depending on your needs.

Under the Hood, it uses the easy to implement [Speck Cipher][speck] and
[Base58][base58] encoding.

[speck]: https://en.wikipedia.org/wiki/Speck_%28cipher%29
[base58]: https://en.wikipedia.org/wiki/Base58
