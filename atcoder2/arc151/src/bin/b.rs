#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use ac_library_rs::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    ops::AddAssign,
};

fn main() {
    input! {
        n: usize,
        m: usize,
        pp: [Usize1; n],
    };
    let mut rs = ModInt998244353::new(0);
    let mut group_cnt = n as u64;
    let mm = ModInt998244353::new(m);
    let mul = mm * (mm - 1) / 2;
    // dbg!(mul);
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        let p = pp[i];
        if !uf.equiv(i, p) {
            if i != p {
                rs += mul * mm.pow(group_cnt.saturating_sub(2));
                // dbg!(mul * mm.pow(group_cnt.saturating_sub(2) as u64));
            }
            uf.union(i, p);
            group_cnt -= 1;
        }
    }

    println!("{rs}");
}
