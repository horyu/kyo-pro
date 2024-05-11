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
        aa: [isize; n],
    };
    let mut ft = ac_library::FenwickTree::new(n, 0isize);
    for (i, &a) in aa.iter().enumerate() {
        ft.add(i, a);
    }
    let a2j = aa
        .iter()
        .copied()
        .sorted_unstable()
        .dedup()
        .enumerate()
        .map(|(i, a)| (a, i))
        .collect::<BTreeMap<_, _>>();
    let len = a2j.len();
    let mut jj = ac_library::FenwickTree::new(len, 0isize);
    for (i, &a) in aa.iter().enumerate() {
        let j = a2j[&a];
        jj.add(j, 1);
    }
    let mut rs = 0;
    for (i, a) in aa[..(n - 1)].iter().copied().enumerate() {
        let j = a2j[&a];
        jj.add(j, -1);
        rs += (n - 1 - i) as isize * a + ft.sum((i + 1)..);
        // eprintln!("{i} {a} +{}", ft.sum((i + 1)..));

        // a + aj > 1e8 となる aj の個数
        let min_aj = 100_000_000 - a;
        if let Some((&k, &v)) = a2j.range(min_aj..).next() {
            rs -= 100_000_000 * jj.sum(v..);
            // eprintln!("{i} {a} -{}", 100_000_000 * jj.sum(v..));
        }
    }
    println!("{rs}");
}
