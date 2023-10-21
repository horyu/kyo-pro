#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use nalgebra::coordinates::X;
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
    const MAX: usize = 2e5 as usize;

    let mut ft1 = ac_library::FenwickTree::new(MAX + 1, 0usize);
    let mut ft2 = ac_library::FenwickTree::new(MAX + 1, 0usize);

    let mut sum = ModInt998244353::default();
    for (i, a) in aa.iter().copied().enumerate() {
        let ma = ModInt998244353::new(a);
        let d = ma * ft1.sum(0..=a) + ft2.sum((a + 1)..);
        sum += d * 2 + ma;
        let rs = sum / (i + 1).pow(2);
        println!("{rs}");

        ft1.add(a, 1);
        ft2.add(a, a);
    }
}
