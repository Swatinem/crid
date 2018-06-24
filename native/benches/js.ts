import readline from "readline";
import {js, neon} from "../../src";

const NS_PER_SEC = 1e9;

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

let key = Uint32Array.of(0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100);

const Crid = process.argv[2] === "neon" ? neon : js;
const crid = new Crid(key);

rl.on("line", input => {
  const num = Number(input);
  const start = process.hrtime();
  for (let i = 0; i < num; i++) {
    const hi = 0x3b726574;
    const lo = 0x7475432d;
    const encoded = crid.encode(hi, lo);
    const decoded = crid.decode(encoded);
    decoded[0] === hi && decoded[1] === lo;
  }
  const diff = process.hrtime(start);
  const time = diff[0] * NS_PER_SEC + diff[1];
  console.log(time);
});
