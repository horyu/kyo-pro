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
        aa: [usize; n],
    };
    let mut hm = HashMap::new();
    hm.insert(0, 1);

    let mut rs = 0usize;
    let mut s = 0;
    for a in aa {
        s = (s + a) % m;
        let e = hm.entry(s).or_default();
        rs += *e;
        *e += 1;
    }
    println!("{rs}");
}
