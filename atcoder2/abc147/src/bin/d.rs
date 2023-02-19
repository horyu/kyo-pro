#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library_rs::ModInt1000000007;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    };
    let mut rs = ModInt1000000007::new(0);
    for i in 0..60 {
        let mut cc = [0usize; 2];
        for a in aa.iter().copied() {
            cc[(a >> i) & 1] += 1;
        }
        rs += ModInt1000000007::new(cc[0] * cc[1]) * (1usize << i);
    }
    println!("{rs}");
}
