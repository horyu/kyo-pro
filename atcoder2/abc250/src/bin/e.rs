#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
#![feature(map_try_insert)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
        q: usize,
        xxyy: [(usize, usize); q],
    };
    // https://atcoder.jp/contests/abc250/submissions/33595407
    let mut ahm = HashMap::new();
    for (i, a) in aa.iter().copied().enumerate() {
        let _ = ahm.try_insert(a, i);
    }
    let mut bhm = HashMap::new();
    for (i, b) in bb.iter().copied().enumerate() {
        let _ = bhm.try_insert(b, i);
    }
    let mut a2b = vec![0; n + 1];
    let mut b2a = vec![0; n + 1];
    for i in 0..n {
        a2b[i + 1] = a2b[i];
        if let Some(j) = bhm.get(&aa[i]).copied() {
            a2b[i + 1] = a2b[i + 1].max(j);
        } else {
            a2b[i + 1] = n;
        }
        b2a[i + 1] = b2a[i];
        if let Some(j) = ahm.get(&bb[i]).copied() {
            b2a[i + 1] = b2a[i + 1].max(j);
        } else {
            b2a[i + 1] = n;
        }
    }
    for (x, y) in xxyy {
        let rs = ["No", "Yes"][(a2b[x] < y && b2a[y] < x) as usize];
        println!("{rs}");
    }
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
        q: usize,
        xxyy: [(Usize1, Usize1); q],
    };
    let mut bhm = HashMap::new();
    for (i, b) in bb.iter().copied().enumerate() {
        let _ = bhm.try_insert(b, i);
    }
    let mut hs = HashSet::new();
    let mut ahs = HashSet::new();
    let mut bhs = HashSet::new();
    let mut uf = UnionFind::new(2 * n);
    let mut j = 0;
    bhs.insert(bb[0]);
    for (i, a) in aa.iter().copied().enumerate() {
        if hs.contains(&a) {
            if 0 < i && ahs.is_empty() && bhs.is_empty() {
                uf.union(i - 1, i);
                // eprintln!("{i} {j} {:?}", (i - 1, i));
            }
        } else if bhs.remove(&a) {
            hs.insert(a);
        } else {
            ahs.insert(a);
        }

        while !ahs.is_empty() {
            if let Some(b) = bb.get(j + 1).copied() {
                j += 1;
                if hs.contains(&b) {
                    //
                } else if ahs.remove(&b) {
                    hs.insert(b);
                } else {
                    bhs.insert(b);
                }
            }
            if j == n - 1 {
                break;
            }
        }

        if ahs.is_empty() && bhs.is_empty() {
            uf.union(i, j + n);
            // eprintln!("_{i} {j} {:?}", (i, j + n));
            while let Some(b) = bb.get(j + 1).copied() {
                if hs.contains(&b) {
                    j += 1;
                    uf.union(i, j + n);
                    // eprintln!("__{i} {j} {:?}", (i, j + n));
                    continue;
                }
                break;
            }
        }
        // eprintln!("@{i} {j} {ahs:?} {bhs:?} {hs:?}");
    }
    for (x, y) in xxyy {
        let rs = ["No", "Yes"][uf.equiv(x, y + n) as usize];
        println!("{rs}");
    }
}
