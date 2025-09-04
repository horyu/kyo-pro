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
        n: usize,
        qq: [usize; n],
        aa: [usize; n],
        bb: [usize; n],
    };
    let mut rs = 0usize;
    let cnt_max_a = izip!(&qq, &aa)
        .filter_map(|(&q, &a)| q.checked_div(a))
        .min()
        .unwrap();
    for cnt_a in 0..=cnt_max_a {
        let cnt_b = izip!(&qq, &aa, &bb)
            .filter_map(|(&q, &a, &b)| (q - cnt_a * a).checked_div(b))
            .min()
            .unwrap();
        rs = rs.max(cnt_a + cnt_b);
    }
    println!("{rs}");
}
