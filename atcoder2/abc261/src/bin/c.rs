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
        ss: [String; n],
    };
    let mut hm = HashMap::new();
    for s in ss {
        if let Some(e) = hm.get_mut(&s) {
            *e += 1;
            println!("{s}({e})");
        } else {
            println!("{s}");
            hm.insert(s, 0usize);
        }
    }
}
