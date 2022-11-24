use rand::seq::SliceRandom;
use rand::{rngs::StdRng, Rng, SeedableRng};

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;

const SEED: u64 = 18446744073709551557;
const N: usize = 1000;

fn bench_insert_hashmap(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut g = c.benchmark_group("insert_hashmap");
    for sz in &[1024, 8 * 1024, 64 * 1024, 512 * 1024, 4096 * 1024] {
        let mut m = std::collections::HashMap::new();
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

fn bench_lookup_hashmap(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut g = c.benchmark_group("lookup_hashmap");
    for sz in &[1024, 8 * 1024, 64 * 1024, 512 * 1024, 4096 * 1024] {
        let mut m: std::collections::HashMap<i64, u64> = std::collections::HashMap::new();
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

criterion_group!(benches, bench_insert_hashmap, bench_lookup_hashmap);
criterion_main!(benches);
