use core::borrow::Borrow;

use core::cmp::Ordering;

use super::node::{marker, ForceResult::*, Handle, NodeRef};

use SearchResult::*;

pub enum SearchResult<BorrowType, K, V, FoundType, GoDownType> {
    Found(Handle<NodeRef<BorrowType, K, V, FoundType>, marker::KV>),
    GoDown(Handle<NodeRef<BorrowType, K, V, GoDownType>, marker::Edge>),
}

pub fn search_tree<BorrowType, K, V, Q: ?Sized>(
    mut node: NodeRef<BorrowType, K, V, marker::LeafOrInternal>,
    key: &Q,
) -> SearchResult<BorrowType, K, V, marker::LeafOrInternal, marker::Leaf>
where
    Q: Ord,
    K: Borrow<Q>,
{
    loop {
        match search_node(node, key) {
            Found(handle) => return Found(handle),
            GoDown(handle) => match handle.force() {
                Leaf(leaf) => return GoDown(leaf),
                Internal(internal) => {
                    node = internal.descend();
                    continue;
                }
            },
        }
    }
}

pub fn search_node<BorrowType, K, V, Type, Q: ?Sized>(
    node: NodeRef<BorrowType, K, V, Type>,
    key: &Q,
) -> SearchResult<BorrowType, K, V, Type, Type>
where
    Q: Ord,
    K: Borrow<Q>,
{
    match search_linear(&node, key) {
        (idx, true) => Found(Handle::new_kv(node, idx)),
        (idx, false) => SearchResult::GoDown(Handle::new_edge(node, idx)),
    }
}

pub fn search_node_at<BorrowType, K, V, Type, Q: ?Sized>(
    handle: Handle<NodeRef<BorrowType, K, V, Type>, marker::Edge>,
    key: &Q,
) -> SearchResult<BorrowType, K, V, Type, Type>
where
    Q: Ord,
    K: Borrow<Q>,
{
    let (node, start) = handle.into_node_and_index();
    match search_linear_at(&node, key, start) {
        (idx, true) => Found(Handle::new_kv(node, idx)),
        (idx, false) => SearchResult::GoDown(Handle::new_edge(node, idx)),
    }
}

pub fn search_linear<BorrowType, K, V, Type, Q: ?Sized>(
    node: &NodeRef<BorrowType, K, V, Type>,
    key: &Q,
) -> (usize, bool)
where
    Q: Ord,
    K: Borrow<Q>,
{
    for (i, k) in node.keys().iter().enumerate() {
        match key.cmp(k.borrow()) {
            Ordering::Greater => {}
            Ordering::Equal => return (i, true),
            Ordering::Less => return (i, false),
        }
    }
    (node.keys().len(), false)
}

pub fn search_linear_at<BorrowType, K, V, Type, Q: ?Sized>(
    node: &NodeRef<BorrowType, K, V, Type>,
    key: &Q,
    start: usize,
) -> (usize, bool)
where
    Q: Ord,
    K: Borrow<Q>,
{
    for (i, k) in node.keys()[start..].iter().enumerate() {
        match key.cmp(k.borrow()) {
            Ordering::Greater => {}
            Ordering::Equal => return (i + start, true),
            Ordering::Less => return (i + start, false),
        }
    }
    (node.keys().len(), false)
}

pub(crate) fn search_tree_many<'a, 'b, K: 'a, V: 'a, Q>(
    node: NodeRef<marker::Immut<'a>, K, V, marker::LeafOrInternal>,
    keys: &'a [Q],
    out: &'b mut alloc::vec::Vec<Option<&'a V>>,
) where
    Q: Ord,
    K: Borrow<Q>,
{
    let mut cur_node = node.first_edge();
    let mut stack = arrayvec::ArrayVec::<_, { super::node::MAX_DEPTH }>::new();

    let mut cur_bound = node.last_edge();
    'next_key: for k in keys {
        while !cur_bound
            .right_kv()
            .map(|right_bound| k < right_bound.into_kv().0.borrow())
            .unwrap_or(true)
        {
            cur_node = cur_bound;
            cur_bound = stack.pop().unwrap();
        }
        'search: loop {
            match search_node_at(cur_node, &k) {
                Found(handle) => {
                    cur_node = handle.left_edge();
                    out.push(Some(handle.into_kv().1));
                    continue 'next_key;
                }
                GoDown(handle) => {
                    cur_node = handle;
                    match handle.force() {
                        Leaf(_) => {
                            out.push(None);
                            continue 'next_key;
                        }
                        Internal(internal) => {
                            if internal.right_kv().is_ok() {
                                stack.push(std::mem::replace(&mut cur_bound, cur_node));
                            };
                            cur_node = internal.descend().first_edge();
                            continue 'search;
                        }
                    }
                }
            }
        }
    }
}
