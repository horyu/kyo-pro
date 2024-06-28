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
        s: Chars,
        q: usize,
        xxyy: [(char, char); q],
    };
    let mut hm = HashMap::new();
    for c in 'a'..='z' {
        hm.insert(c, c);
    }
    for (x, y) in xxyy {
        for (&k, v) in hm.iter_mut() {
            if x == *v {
                *v = y;
            }
        }
    }
    let rs = s
        .into_iter()
        .map(|c| hm.get(&c).copied().unwrap_or(c))
        .join("");
    println!("{rs}");
}
