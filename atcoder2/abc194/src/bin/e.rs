#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
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
        m: usize,
        aa: [usize; n],
    };
    let mut bts = (0..=n).collect::<BTreeSet<_>>();
    let mut vv = vec![0; n + 1];
    for i in 0..(m - 1) {
        let a = aa[i];
        if vv[a] == 0 {
            bts.remove(&a);
        }
        vv[a] += 1;
    }

    let mut rs = !0usize;
    for i in 0..=(n - m) {
        let j = i + m - 1;
        let ai = aa[i];
        let aj = aa[j];
        if vv[aj] == 0 {
            bts.remove(&aj);
        }
        vv[aj] += 1;

        rs = rs.min(bts.iter().min().copied().unwrap());

        vv[ai] -= 1;
        if vv[ai] == 0 {
            bts.insert(ai);
        }
    }

    println!("{rs}");
}
