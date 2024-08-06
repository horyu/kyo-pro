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
        h: usize,
        w: usize,
        m: usize,
        ttaaxx: [(usize, Usize1, usize); m],
    };
    let mut ii = HashSet::<usize>::from_iter(0..h);
    let mut jj = HashSet::<usize>::from_iter(0..w);
    let mut counter = counter::Counter::<_>::new();
    counter[&0] = h * w;
    for (t, a, x) in ttaaxx.into_iter().rev() {
        if t == 1 {
            if ii.remove(&a) {
                counter[&x] += jj.len();
                counter[&0] -= jj.len();
            }
        } else {
            if jj.remove(&a) {
                counter[&x] += ii.len();
                counter[&0] -= ii.len();
            }
        }
    }
    counter.retain(|_k, v| 0 < *v);
    println!("{}", counter.len());
    for (k, v) in counter.into_iter().sorted_unstable_by_key(|kv| kv.0) {
        println!("{k} {v}");
    }
}
