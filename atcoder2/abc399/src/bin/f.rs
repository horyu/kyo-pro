#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

/*
愚直
let mut rs = ModInt998244353::default();
for l in 0..n {
    for r in l..n {
        let mut tmp = ModInt998244353::default();
        for i in l..=r {
            tmp += aa[i];
        }
        rs += tmp.pow(k);
    }
}
println!("{rs}");
*/

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n],
    };
    // let mut rs = ModInt998244353::default();
    // println!("{rs}");
}
