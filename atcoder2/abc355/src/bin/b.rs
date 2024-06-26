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
        bb: [usize; m],
    };
    let cc = chain!(&aa, &bb).copied().sorted_unstable().collect_vec();
    let ahs = aa.iter().copied().collect::<HashSet<_>>();
    for (ci, cj) in cc.into_iter().tuple_windows() {
        if ahs.contains(&ci) && ahs.contains(&cj) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
