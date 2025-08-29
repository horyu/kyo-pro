#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{Itertools, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        mut n: usize,
        m: usize,
        aabb: [(usize, usize); m],
    };
    let ddaabb = aabb
        .into_iter()
        .map(|(a, b)| (a - b, a, b))
        .sorted_unstable_by_key(|x| x.0)
        .collect_vec();
    let mut rs = 0usize;
    for (d, a, b) in ddaabb {
        if n < a {
            continue;
        }
        let k = (n - a) / d + 1;
        rs += k;
        n -= k * d;
    }
    println!("{rs}");
}
