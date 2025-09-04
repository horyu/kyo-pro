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
        t: usize,
        nnddkk: [(usize, usize, usize); t],
    };
    for (n, d, k) in nnddkk {
        let d = d % n;
        if d <= 1 {
            println!("{}", k - 1);
            continue;
        }
        let lcm = n.lcm(&d);
        let w = lcm / d;
        if n <= w {
            println!("{}", d * (k - 1) % n);
        } else {
            println!("{}", (k - 1) / w + d * ((k - 1) % w) % n);
        }
    }
}
