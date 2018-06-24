#[macro_use]
extern crate criterion;
extern crate crid;

use crid::Crid;
use criterion::{black_box, Benchmark, Criterion, Throughput};
use std::process::Command;

fn bench_converter(c: &mut Criterion) {
  let args = [
    "--perf-basic-prof",
    "node_modules/.bin/ts-node",
    "-T",
    "benches/js.ts",
  ];
  let mut node = Command::new("node");
  node.args(&args);
  let mut neon = Command::new("node");
  neon.args(&args);
  neon.arg("neon");
  let mut wasm = Command::new("node");
  wasm.args(&args);
  wasm.arg("wasm");

  let b = Benchmark::new("rust", |b| {
    let key = [0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100];
    let crid = Crid::new(key);
    let block = black_box([0x3b726574, 0x7475432d]);
    b.iter(|| {
      let encoded = crid.encode_to_str(block);
      let decoded = crid.decode(&encoded).unwrap();
      decoded == block
    })
  }).with_program("neon", neon)
    .with_program("js", node)
    .with_program("wasm", wasm)
    .throughput(Throughput::Elements(1));

  c.bench("Crid", b);
}

criterion_group!(benches, bench_converter);
criterion_main!(benches);
