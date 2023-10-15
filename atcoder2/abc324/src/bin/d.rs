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
        n: u32,
        s: Bytes,
    };
    let mut scc = [0; 10];
    for &c in &s {
        scc[(c - b'0') as usize] += 1;
    }
    let mut rs = 0usize;
    for i in 0..=10usize.pow(n).sqrt() {
        let mut ii = i.pow(2);
        let mut icc = [0; 10];
        while 0 < ii {
            icc[ii % 10] += 1;
            ii /= 10;
        }
        if (1..=9).all(|i| scc[i] == icc[i]) {
            rs += 1;
        }
    }
    println!("{rs}");
}
