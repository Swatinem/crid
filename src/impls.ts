import js from "./index";
// @ts-ignore
import { Crid as neon } from "../native";
// @ts-ignore
import { Crid as WasmCrid } from "../wasm/crid_wasm";

class wasm {
  private inner: WasmCrid;
  constructor(key: Array<number>) {
    this.inner = WasmCrid.new(Uint32Array.from(key));
  }
  encode(num1: number, num2: number = 0) {
    return this.inner.encode(num1, num2);
  }
  decode(s: string) {
    const res = this.inner.decode(s);
    if (!res[0]) return undefined;
    return [res[1], res[2]];
  }
}

export { js, neon, wasm };
