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
        m: usize,
        aaa: [[usize; m]; n],
    };
    let hm: HashMap<usize, usize> = aaa
        .iter()
        .flatten()
        .sorted_unstable()
        .enumerate()
        .map(|(idx, &a)| (a, idx))
        .collect();
    let aaa = aaa
        .into_iter()
        .map(|aa| {
            aa.into_iter()
                .map(|a| hm.get(&a).copied().unwrap())
                .sorted_unstable()
                .collect_vec()
        })
        .collect_vec();
    // for aa in &aaa {
    //     eprintln!("{aa:?}");
    // }
    let mut rs = 0usize;
    let mut ft = ac_library::FenwickTree::new(n * m, 0);
    for (i, aa) in aaa.into_iter().enumerate().take(n - 1) {
        rs += (n - 1 - i) * m * (m + 1) / 2;
        for a in aa {
            rs += a - ft.sum(..=a);
            ft.add(a, 1);
        }
    }
    println!("{rs}");
}
