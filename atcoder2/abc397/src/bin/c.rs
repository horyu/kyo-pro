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
    let mut cl = counter::Counter::<_>::new();
    let mut cr = counter::Counter::<_>::new();
    for a in &aa {
        cr[a] += 1;
    }
    let mut rs = 0;
    for (i, a) in aa.iter().enumerate() {
        cr[a] -= 1;
        if cr[a] == 0 {
            cr.remove(a);
        }
        cl[a] += 1;
        rs = rs.max(cl.len() + cr.len());
    }
    println!("{rs}");
}
