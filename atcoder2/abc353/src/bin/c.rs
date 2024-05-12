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
        mut aa: [usize; n],
    };
    aa.sort_unstable();
    let ss = aa.iter().cumsum::<usize>().collect_vec();
    // eprintln!("{ss:?}");
    // eprintln!("{aa:?}");
    let mut rs = 0;
    for (i, ai) in aa.iter().copied().enumerate() {
        rs += ss[n - 1] - ss[i] + ai * (n - 1 - i);
        // 1e8 <= ai + aj となるj>iの個数 * 1e8 を引く
        let j = aa.partition_point(|&aj| ai + aj < 100_000_000).max(i + 1);
        // eprintln!("{ai} {i} {j}");
        rs -= 100_000_000 * (n - j);
    }
    println!("{rs}");
}
