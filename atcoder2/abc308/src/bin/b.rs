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
        cc: [String; n],
        dd: [String; m],
        pp: [usize; m + 1],
    };
    let mut hm = HashMap::new();
    for (i, d) in dd.into_iter().enumerate() {
        hm.insert(d, pp[i + 1]);
    }
    let mut rs = 0;
    for c in cc {
        rs += hm.get(&c).copied().unwrap_or(pp[0]);
    }
    println!("{rs}");
}
