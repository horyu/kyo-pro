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

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    };
    let mut rs = ModInt998244353::default();
    let mut sum = ModInt998244353::default();
    let div = ModInt998244353::new(1) / n;
    for (i, a) in aa.iter().copied().enumerate() {
        let tmp = div * ((div + 1).pow(i as u64) * a + sum);
        rs += tmp * div * (i + 1);
        sum += tmp;
    }
    println!("{rs}");
}
