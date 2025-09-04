#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use rand::Rng;
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        q: usize,
        aa: [Usize1; n],
        bb: [Usize1; n],
        qq: [(Usize1, Usize1, Usize1, Usize1); q],
    };
    // https://atcoder.jp/contests/abc367/editorial/10692
    // Zobrist Hash
    const MOD: usize = (1 << 61) - 1;
    let mut rng = rand::thread_rng();
    let hh = (0..n).map(|_| rng.gen_range(1..MOD)).collect_vec();
    let mut xx = vec![0; n + 1];
    let mut yy = vec![0; n + 1];
    for (i, h) in hh.iter().copied().enumerate() {
        xx[i + 1] = (xx[i] + hh[aa[i]]) % MOD;
        yy[i + 1] = (yy[i] + hh[bb[i]]) % MOD;
    }
    for (l0, r0, l1, r1) in qq {
        let tf = (MOD + xx[r0 + 1] - xx[l0]) % MOD == (MOD + yy[r1 + 1] - yy[l1]) % MOD;
        let rs = ["No", "Yes"][tf as usize];
        println!("{rs}");
    }
}
