#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
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
        q: usize,
        xxyyzzww: [(Usize1, Usize1, Usize1, usize); q],
    };
    let vvv = (0..n)
        .map(|_| [0, 1])
        .multi_cartesian_product()
        .collect_vec();
    let mut rs = ModInt1000000007::new(1);
    for j in 0..60 {
        let mut tmp = 0;
        for vv in &vvv {
            if xxyyzzww
                .iter()
                .all(|&(x, y, z, w)| vv[x] | vv[y] | vv[z] == (w >> j) & 1)
            {
                tmp += 1;
            }
        }
        rs *= tmp;
    }
    println!("{rs}");
}
