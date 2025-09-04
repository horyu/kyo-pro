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
        a: usize,
        b: usize,
        dd: [Usize1; n],
    };
    let ab = a + b;
    let mut dd = dd
        .into_iter()
        .map(|d| d % ab)
        .sorted_unstable()
        .collect_vec();
    for i in 0..n {
        let j = (i + n - 1) % n;
        let diff = dd[i].abs_diff(dd[j]);
        if diff < a {
            println!("Yes");
            return;
        }
        dd[i] += ab;
    }
    println!("No");
}
