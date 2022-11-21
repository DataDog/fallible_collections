use crate::btree::BTreeMap;

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
