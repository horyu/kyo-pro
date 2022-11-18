#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    };
    let mut vv = vec![!0usize; 2 * n + 2];
    vv[1] = 0;
    for (i, a) in izip!(1..=n, aa) {
        // eprintln!("{i} {a}");
        let next = vv[a] + 1;
        vv[2 * i] = next;
        vv[2 * i + 1] = next;
    }
    let rs = vv.iter().skip(1).join("\n");
    println!("{rs}");
}
