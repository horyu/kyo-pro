#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        ttwwss: [(usize, (usize, usize)); m],
    };
    let mut btm = BTreeMap::from_iter(ttwwss);
    let rs = (0..n)
        .map(|_| {
            let mut tmp = 0;
            let mut now = 0;
            while let Some((&t, &(w, s))) = btm.range(now..).next() {
                tmp += w;
                now = t + s;
                btm.remove(&t);
            }
            tmp
        })
        .join("\n");
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        m: usize,
        ttwwss: [(usize, usize, usize); m],
    };
    let mut mm = btreemultimap::BTreeMultiMap::new();
    let mut bts = BTreeSet::from_iter(0..n);
    let mut rs = vec![0usize; n];
    for (t, w, s) in ttwwss {
        let mut remove_keys = HashSet::new();
        for (&k, &i) in mm.range(..=t) {
            remove_keys.insert(k);
            bts.insert(i);
        }
        for k in remove_keys {
            mm.remove(&k);
        }
        if let Some(eater) = bts.pop_first() {
            rs[eater] += w;
            bts.remove(&eater);
            mm.insert(t + s, eater);
        }
    }
    for rs in rs {
        println!("{rs}");
    }
}
