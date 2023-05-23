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
        mut t: usize,
        aa: [usize; n],
    };
    let sum = aa.iter().sum::<usize>();
    t %= sum;
    for (i, a) in aa.iter().copied().enumerate() {
        if t <= a {
            println!("{} {}", i + 1, t);
            return;
        }
        t -= a;
    }
}
