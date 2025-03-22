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
        pp: [Usize1; n],
    };
    let mut ft = ac_library::FenwickTree::new(n, 0usize);
    let mut vv = vec![0; n];
    for (i, p) in pp.into_iter().enumerate().rev() {
        let pos = ft.sum(..=p) + p;
        vv[pos] = i + 1;
        ft.add(p, 1);
        eprintln!("{vv:?} {}", (0..n).map(|i| ft.sum(i..=i)).join(" "));
    }
    let rs = vv.iter().join(" ");
    println!("{rs}");
}
