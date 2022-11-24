use fallible_collections::btree::BTreeMap;
use rand::seq::SliceRandom;
use rand::{rngs::StdRng, Rng, SeedableRng};

use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{black_box, criterion_group};

type Key = i64;
type Value = u64;

const SEED: u64 = 18446744073709551557;
const N: usize = 1000;

fn bench_insert(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut insert_g = c.benchmark_group("insert");
    for sz in &[1024, 8 * 1024, 64 * 1024, 512 * 1024, 4096 * 1024] {
        let mut m: BTreeMap<Key, Value> = BTreeMap::new();
        let mut keys = vec![];
        for i in 0..*sz {
            let k = rng.gen();
            m.try_insert(k, i).expect("insert succeeded");
            keys.push(k);
        }
        insert_g.throughput(criterion::Throughput::Elements(N as u64));
        insert_g.bench_with_input(BenchmarkId::from_parameter(*sz), sz, |b, _| {
            let keys: Vec<i64> = (0..N).map(|_| rng.gen()).collect();
            b.iter(|| {
                for (i, k) in keys.iter().enumerate() {
                    m.try_insert(*k, i as u64).expect("insert succeeded");
                }
            })
        });
    }
}

fn bench_insert_sorted(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut insert_g = c.benchmark_group("insert_sorted");
    for sz in &[1024, 8 * 1024, 64 * 1024, 512 * 1024, 4096 * 1024] {
        let mut m: BTreeMap<Key, Value> = BTreeMap::new();
        let mut keys = vec![];
        for i in 0..*sz {
            let k = rng.gen();
            m.try_insert(k, i).expect("insert succeeded");
            keys.push(k);
        }
        insert_g.throughput(criterion::Throughput::Elements(N as u64));
        insert_g.bench_with_input(BenchmarkId::from_parameter(*sz), sz, |b, _| {
            let mut kvs: Vec<(i64, u64)> = (0..N).map(|_| (rng.gen(), rng.gen())).collect();
            kvs.sort();
            b.iter(|| {
                m.try_insert_many(kvs.iter().cloned());
            })
        });
    }
}

fn bench_insert_sorted_dense(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut insert_g = c.benchmark_group("insert_sorted_dense");
    for sz in &[1024, 8 * 1024, 64 * 1024, 512 * 1024, 4096 * 1024] {
        let mut m: BTreeMap<Key, Value> = BTreeMap::new();
        let mut keys = vec![];
        for i in 0..*sz {
            let k = rng.gen();
            m.try_insert(k, i).expect("insert succeeded");
            keys.push(k);
        }
        insert_g.throughput(criterion::Throughput::Elements(N as u64));
        insert_g.bench_with_input(BenchmarkId::from_parameter(*sz), sz, |b, _| {
            let start: i64 = rng.gen();
            let kvs: Vec<(i64, u64)> = (0..N).map(|i| (start + i as i64 * 7, rng.gen())).collect();
            b.iter(|| {
                m.try_insert_many(kvs.iter().cloned())
                    .expect("insert succeeded");
            })
        });
    }
}

fn bench_lookup(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut g = c.benchmark_group("lookup");
    for sz in &[1024, 8 * 1024, 64 * 1024, 512 * 1024, 4096 * 1024] {
        let mut m: BTreeMap<Key, Value> = BTreeMap::new();
        let mut keys = vec![];
        for i in 0..*sz {
            let k = rng.gen();
            m.try_insert(k, i).expect("insert succeeded");
            keys.push(k);
        }
        keys.shuffle(&mut rng);
        g.throughput(criterion::Throughput::Elements(N as u64));
        g.bench_with_input(BenchmarkId::from_parameter(*sz), sz, |b, _| {
            b.iter(|| {
                for k in keys[..N].iter() {
                    black_box(m.get(k));
                }
            })
        });
    }
}

fn bench_lookup_sorted(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut g = c.benchmark_group("lookup_sorted");
    for sz in &[1024, 8 * 1024, 64 * 1024, 512 * 1024, 4096 * 1024] {
        let mut m: BTreeMap<Key, Value> = BTreeMap::new();
        let mut keys = vec![];
        for i in 0..*sz {
            let k = rng.gen();
            m.try_insert(k, i).expect("insert succeeded");
            keys.push(k);
        }
        keys.shuffle(&mut rng);
        let mut keys_sorted = keys[..N].iter().cloned().collect::<Vec<_>>();
        keys_sorted.sort();
        g.throughput(criterion::Throughput::Elements(N as u64));
        g.bench_with_input(BenchmarkId::from_parameter(*sz), sz, |b, _| {
            b.iter(|| {
                let mut out = Vec::with_capacity(keys_sorted.len());
                m.get_many(&keys_sorted, &mut out);
                black_box(out);
            })
        });
    }
}

fn bench_lookup_sorted_dense(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut g = c.benchmark_group("lookup_sorted_dense");
    for sz in &[1024, 8 * 1024, 64 * 1024, 512 * 1024, 4096 * 1024] {
        let mut m: BTreeMap<Key, Value> = BTreeMap::new();
        let mut keys = vec![];
        for i in 0..*sz {
            let k = rng.gen();
            m.try_insert(k, i).expect("insert succeeded");
            keys.push(k);
        }
        keys.sort();
        let start = rng.gen_range(0..keys.len() - N);
        let keys = &keys[start..start + N];
        g.throughput(criterion::Throughput::Elements(N as u64));
        g.bench_with_input(BenchmarkId::from_parameter(*sz), sz, |b, _| {
            b.iter(|| {
                let mut out = Vec::with_capacity(keys.len());
                m.get_many(&keys, &mut out);
            })
        });
    }
}

fn bench_insert_std(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut g = c.benchmark_group("insert_std");
    for sz in &[1024, 8 * 1024, 64 * 1024, 512 * 1024, 4096 * 1024] {
        let mut m = std::collections::BTreeMap::new();
        for i in 0..*sz {
            let k = rng.gen();
            m.insert(k, i);
        }
        g.throughput(criterion::Throughput::Elements(N as u64));
        g.bench_with_input(BenchmarkId::from_parameter(*sz), sz, |b, _| {
            let keys: Vec<i64> = (0..N).map(|_| rng.gen()).collect();
            b.iter(|| {
                for (i, k) in keys.iter().enumerate() {
                    m.insert(*k, i as u64);
                }
            })
        });
    }
}

fn bench_lookup_std(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut g = c.benchmark_group("lookup_std");
    for sz in &[1024, 8 * 1024, 64 * 1024, 512 * 1024, 4096 * 1024] {
        let mut m: std::collections::BTreeMap<i64, u64> = std::collections::BTreeMap::new();
        let mut keys = vec![];
        for i in 0..*sz {
            let k = rng.gen();
            m.insert(k, i);
            keys.push(k);
        }
        keys.shuffle(&mut rng);
        g.throughput(criterion::Throughput::Elements(N as u64));
        g.bench_with_input(BenchmarkId::from_parameter(*sz), sz, |b, _| {
            b.iter(|| {
                for k in keys[..N].iter() {
                    m.get(k);
                }
            })
        });
    }
}

criterion_group!(
    benches,
    bench_insert,
    bench_insert_sorted,
    bench_insert_sorted_dense,
    bench_insert_std,
    bench_lookup,
    bench_lookup_sorted,
    bench_lookup_sorted_dense,
    bench_lookup_std
);
criterion_main!(benches);
