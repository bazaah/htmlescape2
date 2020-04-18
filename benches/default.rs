extern crate criterion;
extern crate htmlescape2;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use htmlescape2::*;

static BIG_STR: &'static str = include_str!("../benches/assets/moonstone-short.txt");
static LEN: u64 = BIG_STR.len() as u64;

fn bench_encode_attribute(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_throughput");
    group.throughput(Throughput::Bytes(LEN));
    group.bench_function("encode attribute", |b| {
        b.iter(|| encode_attribute(black_box(BIG_STR)))
    });
    group.finish();
}

fn bench_encode_minimal(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_throughput");
    group.throughput(Throughput::Bytes(LEN));
    group.bench_function("encode minimal", |b| {
        b.iter(|| encode_minimal(black_box(BIG_STR)))
    });
    group.finish();
}

fn bench_decode_attribute(c: &mut Criterion) {
    let encoded = encode_attribute(BIG_STR);

    let mut group = c.benchmark_group("decode_throughput");
    group.throughput(Throughput::Bytes(LEN));
    group.bench_function("decode attribute", |b| {
        b.iter(|| decode_html(black_box(&encoded)))
    });
    group.finish();
}

fn bench_decode_minimal(c: &mut Criterion) {
    let encoded = encode_minimal(BIG_STR);

    let mut group = c.benchmark_group("decode_throughput");
    group.throughput(Throughput::Bytes(LEN));
    group.bench_function("decode minimal", |b| {
        b.iter(|| decode_html(black_box(&encoded)))
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_encode_attribute,
    bench_encode_minimal,
    bench_decode_attribute,
    bench_decode_minimal
);
criterion_main!(benches);
