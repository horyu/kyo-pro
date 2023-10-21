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
        m: usize,
        t: usize,
        aa: [usize; n - 1],
        xxyy: [(Usize1, usize); m]
    };
    let mut add = vec![0; n];
    for (x, y) in xxyy {
        add[x] += y;
    }
    let mut crr = t;
    for i in 0..(n - 1) {
        crr += add[i];
        if crr <= aa[i] {
            println!("No");
            return;
        }
        crr -= aa[i];
    }
    println!("Yes");
}
