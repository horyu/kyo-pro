#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    };
    let mut mm = multimap::MultiMap::new();
    for (i, a) in aa.iter().copied().enumerate() {
        mm.insert(a, i);
    }
    let mut rs = 0usize;
    for i in 1..=n {
        // 幅 i の区間に線は i/2 本ある
        rs += (n + 1 - i) * (i / 2);
    }
    for (_, pp) in mm {
        let mut l = 0;
        let mut r = pp.len().saturating_sub(1);
        // eprintln!("{l} {r} {pp:?}");
        while l < r {
            // (pp[l], pp[r]), (pp[l]-2, pp[r]+2), (pp[l]-2, pp[r]+2), ...
            if pp[l] + 1 < n - pp[r] {
                // eprintln!("u {l} {r} {}*{}", (r - l), (pp[l] + 1));
                rs -= (r - l) * (pp[l] + 1);
                l += 1;
            } else {
                // eprintln!("d {l} {r} {}*{}", (r - l), (n - pp[r]));
                rs -= (r - l) * (n - pp[r]);
                r -= 1;
            }
        }
    }
    println!("{rs}");
}
