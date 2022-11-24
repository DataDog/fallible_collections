use crate::btree::BTreeMap;
use rand::{rngs::StdRng, Rng, SeedableRng};

const SEED: u64 = 18446744073709551557;

type Key = i64;
type Value = u64;

#[test]
fn test_lookup_sorted() {
    let mut m = BTreeMap::new();
    for _ in 0..2 {
        for i in 0..64 {
            m.try_insert(i as Key, i as Value).unwrap();
        }
    }
    assert_eq!(64, m.len());
    assert_eq!(
        (0..64).map(|i| (i as Key, i as Value)).collect::<Vec<_>>(),
        m.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>()
    );
    let mut out = vec![];
    let range = (0..64).collect::<Vec<_>>();
    m.get_many(&range, &mut out);
    assert_eq!(
        (0..64).map(|v| Some(v)).collect::<Vec<_>>(),
        out.iter().map(|x| x.cloned()).collect::<Vec<_>>()
    );
}

#[test]
fn test_insert_sorted() {
    let mut m = BTreeMap::new();
    for _ in 0..2 {
        let kvs = (0..64).map(|i| (i as Key, i as Value));
        m.try_insert_many(kvs).expect("insert succeeded");
    }
    assert_eq!(64, m.len());
    assert_eq!(
        (0..64).map(|i| (i as Key, i as Value)).collect::<Vec<_>>(),
        m.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>(),
    );
    let mut out = vec![];
    let range = (0..64).collect::<Vec<_>>();
    m.get_many(&range, &mut out);
    assert_eq!(
        (0u64..64).map(|v| Some(v)).collect::<Vec<_>>(),
        out.iter().map(|o| o.cloned()).collect::<Vec<_>>()
    );
    // println!("{:#?}", out);
}

#[test]
fn randomized_insert_sorted() {
    let mut rng = StdRng::seed_from_u64(SEED);
    #[cfg(miri)]
    let n = 64..65;
    #[cfg(not(miri))]
    let n = 0..512;
    for k in n {
        let mut m = BTreeMap::new();
        let mut vx: Vec<(Key, Value)> = vec![];
        for i in 0..k {
            let k = rng.gen();
            vx.push((k, i));
        }
        vx.sort_by(|a, b| a.0.cmp(&b.0));
        m.try_insert_many(vx.iter().cloned())
            .expect("insert succeeded");
        let mut j = 1;
        for i in 1..vx.len() {
            if vx[i - 1].0 == vx[i].0 {
                vx[j - 1] = vx[i];
            } else {
                vx[j] = vx[i];
                j += 1;
            }
        }
        vx.truncate(j);
        assert_eq!(
            vx,
            m.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<Vec<_>>()
        );
        assert_eq!(vx.len(), m.len());
        for (k, v) in &vx {
            assert_eq!(*v, *m.get(k).unwrap());
        }
        let mut out = vec![];
        let keys = vx.iter().map(|kv| kv.0).collect::<Vec<_>>();
        m.get_many(&keys, &mut out);
        for i in 0..vx.len() {
            assert_eq!(Some(vx[i].1), out[i].cloned());
        }
    }
}

#[cfg(miri)]
const N: usize = 100;

#[cfg(not(miri))]
const N: usize = 1000;

#[test]
fn test_insert_sorted_rnd() {
    let mut rng = StdRng::seed_from_u64(SEED);
    #[cfg(miri)]
    let sizes = &[64];
    #[cfg(not(miri))]
    let sizes = &[1024, 8 * 1024, 64 * 1024, 512 * 1024, 4096 * 1024];
    for sz in sizes {
        let mut m = BTreeMap::new();
        let mut keys = vec![];
        for i in 0..*sz {
            let k = rng.gen();
            m.try_insert(k, i).expect("insert succeeded");
            keys.push(k);
        }
        #[cfg(miri)]
        let k = 2;
        #[cfg(not(miri))]
        let k = 1000;
        for _ in 0..k {
            let mut kvs: Vec<(i64, u64)> = (0..N).map(|_| (rng.gen(), rng.gen())).collect();
            kvs.sort();
            m.try_insert_many(kvs).expect("insert succeeded");
        }
    }
}
