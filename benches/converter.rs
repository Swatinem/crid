#[macro_use]
extern crate criterion;
extern crate crid;

use crid::Crid;
use criterion::{black_box, Benchmark, Criterion, Throughput};
use std::process::Command;

fn bench_converter(c: &mut Criterion) {
  let mut node_js = Command::new("ts-node");
  node_js.arg("-T");
  node_js.arg("benches/js.ts");

  let b = Benchmark::new_external("node_js", node_js)
    .with_function("rust", |b| {
      let key = [0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100];
      let crid = Crid::new(key);
      let block = black_box([0x3b726574, 0x7475432d]);
      b.iter(|| {
        let encoded = crid.encode_str(block);
        let decoded = crid.decode(&encoded).unwrap();
        decoded == block
      })
    })
    .throughput(Throughput::Bytes(16));

  c.bench("Base conversion", b);
}

criterion_group!(benches, bench_converter);
criterion_main!(benches);
