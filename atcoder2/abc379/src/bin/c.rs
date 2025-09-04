#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        m: usize,
        xx: [usize; m],
        aa: [usize; m],
    };
    if aa.iter().sum::<usize>() != n {
        println!("-1");
        return;
    }
    let mut btm = BTreeMap::new();
    for (x, a) in izip!(xx, aa) {
        btm.insert(x, a);
    }
    let mut pos = 1;
    let mut rs = 0usize;
    while let Some((k, v)) = btm.pop_first() {
        // eprintln!("{pos} {k} {:?}", pos.cmp(&k));
        match pos.cmp(&k) {
            Ordering::Less => {
                println!("-1");
                return;
            }
            Ordering::Equal => {
                // v-1個の石をpos以上の位置に移動する => 1..=(v-1).sum
                rs += v * (v - 1) / 2;
                pos += v;
            }
            Ordering::Greater => {
                // v個の石をkからpos以上の位置に移動する => (pos-k)..=(pos-k+v-1).sum
                rs += (2 * pos - 2 * k + v - 1) * v / 2;
                pos += v;
            }
        }
    }
    println!("{rs}");
}
