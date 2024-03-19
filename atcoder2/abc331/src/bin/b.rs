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
        s: usize,
        m: usize,
        l: usize,
    };
    let mut rs = usize::MAX;
    for a in 0..100 {
        for b in 0..100 {
            for c in 0..100 {
                let cnt = 6 * a + 8 * b + 12 * c;
                if n <= cnt {
                    rs = rs.min(a * s + b * m + c * l);
                }
            }
        }
    }
    println!("{rs}");
}
