export function convert32(
  source: Uint32Array,
  sourceBase: number,
  dest: Uint32Array,
  destBase: number,
) {
  let length = dest.length;
  for (const digit of source) {
    let carry = digit;
    let wrote_length = 0;
    for (let i = dest.length - 1; i >= 0; i--) {
      if (carry == 0 && wrote_length >= length) {
        break;
      }
      carry += sourceBase * dest[i];
      dest[i] = carry % destBase;
      carry = Math.floor(carry / destBase);
      wrote_length += 1;
    }
    length = wrote_length;
  }
}
