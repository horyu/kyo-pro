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
        aa: [usize; n],
    };
    let mut mm = btreemultimap::BTreeMultiMap::new();
    for (i, a) in aa.iter().copied().enumerate() {
        mm.insert(a, i);
    }
    let mut rs = 0usize;
    let mut ft = ac_library::FenwickTree::new(n, 0usize);
    let mut cc = ac_library::FenwickTree::new(n, 0usize);
    for (a, ii) in mm {
        // eprintln!("{a} {ii:?}");
        for &i in &ii {
            rs += a * cc.sum(..i) - ft.sum(..i);
        }
        for &i in &ii {
            ft.add(i, a);
            cc.add(i, 1);
        }
    }

    println!("{rs}");
}
