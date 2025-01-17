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
        m: usize,
        aa: [usize; n],
    };
    // https://atcoder.jp/contests/abc378/editorial/11289
    let mut rs = 0usize;
    let mut ft = ac_library::FenwickTree::new(m, 0usize);
    let ss = chain!([0], aa).cumsum::<usize>().map(|s| s % m).collect_vec();
    let mut tmp = 0;
    for r in 0..=n {
        rs += ss[r] * r + ft.sum((ss[r] + 1)..) * m - tmp;
        tmp += ss[r];
        ft.add(ss[r], 1);
    }
    println!("{rs}");
}
