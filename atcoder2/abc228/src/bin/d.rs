#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        q: usize,
        ttxx: [(usize, usize); q],
    };
    const N: usize = 2usize.pow(20);
    let mut bts: BTreeSet<usize> = (0..N).collect();
    let mut aa = vec![None; N];
    for (t, x) in ttxx {
        if t == 1 {
            let i = if let Some(&i) = bts.range((x % N)..).next() {
                i
            } else {
                bts.pop_first().unwrap()
            };
            bts.remove(&i);
            aa[i] = Some(x);
        } else {
            if let Some(a) = aa[x % N] {
                println!("{a}");
            } else {
                println!("-1");
            }
        }
    }
}
