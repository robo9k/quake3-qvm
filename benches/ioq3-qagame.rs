#![feature(test)]

extern crate test;
extern crate quake3_qvm;

use test::Bencher;
use quake3_qvm::*;

#[bench]
fn bench_parse_qvm_ioq3_qagame(b: &mut Bencher) {
    let data = include_bytes!("../assets/ioq3/baseq3/vm/qagame.qvm");
    b.iter(|| parser::parse_qvm(data).unwrap());
}
