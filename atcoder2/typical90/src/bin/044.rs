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
        q: usize,
        mut aa: [usize; n],
        ttxxyy: [(usize, usize, usize); q]
    };
    let mut offset = 0;
    for (t, x, y) in ttxxyy {
        match t {
            1 => {
                aa.swap((x - 1 + n - offset) % n, (y - 1 + n - offset) % n);
            }
            2 => {
                offset = (offset + 1) % n;
            }
            _ => {
                println!("{}", aa[(x - 1 + n - offset) % n]);
            }
        }
    }
}
