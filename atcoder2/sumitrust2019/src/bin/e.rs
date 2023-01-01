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
    let mut rs = ModInt1000000007::new(1);
    let mut vv = [0; 3];
    for a in aa {
        let ww = vv.iter().positions(|v| *v == a).collect_vec();
        rs *= ww.len();
        if let Some(w) = ww.first().copied() {
            vv[w] += 1;
        }
    }
    // dbg!(rs);
    println!("{rs}");
}
