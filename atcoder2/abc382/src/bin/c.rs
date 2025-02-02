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
    let mut btm = BTreeMap::new();
    for (i, a) in aa.iter().copied().enumerate() {
        if btm.range(..=a).next().is_none() {
            btm.insert(a, i);
        }
    }
    for b in bb {
        if let Some((_, i)) = btm.range(..=b).max() {
            let rs = i + 1;
            println!("{rs}");
        } else {
            println!("-1");
        }
    }
}
