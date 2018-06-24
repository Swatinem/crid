export function convert32(
  source: Array<number>,
  sourceBase: number,
  dest: Array<number>,
  destBase: number,
) {
  let length = 0;
  for (const digit of source) {
    let carry = digit >>> 0;
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
