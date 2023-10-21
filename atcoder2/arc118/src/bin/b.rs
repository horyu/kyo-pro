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
        k: usize,
        n: usize,
        m: usize,
        aa: [usize; k],
    };
    // max(|bi/m - ai/n|)
    let mut bb = aa.iter().copied().map(|a| a * m / n).collect_vec();
    // eprintln!("{}", aa.iter().join(" "));
    // eprintln!("{}", bb.iter().join(" "));
    // dbg!(bb.iter().sum::<usize>());
    let sum = bb.iter().sum::<usize>();
    let ii = (0..k)
        .sorted_unstable_by_key(|&i| (bb[i] * n).abs_diff(aa[i] * m))
        .rev()
        .take(m - sum);
    for i in ii {
        bb[i] += 1;
    }
    let rs = bb.iter().join(" ");
    println!("{rs}");
}
