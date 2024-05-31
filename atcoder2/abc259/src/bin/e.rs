#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
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
    };
    let mut mm = btreemultimap::BTreeMultiMap::new();
    for i in 0..n {
        input! {
            m: usize,
            ppee: [(usize, usize); m],
        };
        for (p, e) in ppee {
            mm.insert(p, (e, i));
        }
    }
    let mut hs = HashSet::new();
    for (_, eeii) in mm {
        let eeii = eeii.into_iter().max_set_by_key(|ei| ei.0);
        if eeii.len() == 1 {
            hs.insert(eeii[0].1);
        }
    }
    // 素因数の最大値を持つ要素数 + 削除しても影響がない要素数
    let rs = hs.len() + usize::from(n != hs.len());
    println!("{rs}");
}
