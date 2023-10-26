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
        xxyy: [(isize, isize); n],
    };
    let mut rs = 0;
    for (a, b, c) in xxyy.into_iter().tuple_combinations() {
        let (x1, y1) = (b.0 - a.0, b.1 - a.1);
        let (x2, y2) = (c.0 - a.0, c.1 - a.1);
        let tmp = (x1 * y2 - x2 * y1).abs();
        if tmp != 0 && tmp % 2 == 0 {
            rs += 1;
        }
    }
    println!("{rs}");
}
