#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use num_traits::Saturating;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        x: usize,
        tt: [usize; n],
    };
    let mut pp = vec![ModInt998244353::new(0); x + 1];
    pp[0] = ModInt998244353::new(1);
    let div = ModInt998244353::new(1) / n;
    let mut rs = ModInt998244353::default();
    if x < tt[0] {
        rs = pp[0] * div;
    }
    for i in 1..=x {
        for (j, t) in tt.iter().copied().enumerate() {
            if t <= i {
                pp[i] = pp[i] + pp[i - t];
            }
        }
        pp[i] *= div;
        if x < i + tt[0] {
            rs += pp[i] * div;
        }
    }
    if cfg!(debug_assertions) {
        let tmp = pp[(x).saturating_sub(tt[0] - 1)..=x]
            .iter()
            .sum::<ModInt998244353>()
            * div;
        dbg!(tmp, rs == tmp);
    }
    println!("{rs}");
}
