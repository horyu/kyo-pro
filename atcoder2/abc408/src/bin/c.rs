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
        llrr: [(Usize1, Usize1); m],
    };
    let mut imos = vec![0isize; n + 1];
    for (l, r) in llrr {
        imos[l] += 1;
        imos[r + 1] -= 1;
    }
    for i in 0..n {
        imos[i + 1] += imos[i];
    }
    let rs = imos[..n].iter().min().unwrap();
    println!("{rs}");
}
