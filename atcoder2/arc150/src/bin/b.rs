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
        aabb: [(isize, isize); n],
    };
    let sqrt = 45000;
    // https://atcoder.jp/contests/arc150/submissions/35551411
    for (a, b) in aabb {
        let mut rs = std::isize::MAX;
        for j in a..=sqrt {
            let x = b.div_ceil(j) * j;
            rs = rs.min((x - b) + (j - a));
        }
        for j in 1..=sqrt {
            let aa = if a * j < b { b.div_ceil(j) } else { a };
            rs = rs.min(aa * j - b + aa - a);
        }
        println!("{rs}");
    }
}
