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
        aabbxxyy: [(Usize1, Usize1, isize, isize); m],
    };
    let mut g = vec![vec![]; n];
    for (a, b, x, y) in aabbxxyy.iter().copied() {
        g[a].push((b, x, y));
        g[b].push((a, -x, -y));
    }
    let mut pp = vec![(!0, !0); n];
    pp[0] = (0, 0);
    let mut qq = VecDeque::new();
    qq.push_back(0);
    let mut pushed = vec![false; n];
    pushed[0] = true;
    while let Some(qi) = qq.pop_front() {
        for &(i, x, y) in g[qi].iter() {
            if pushed[i] {
                continue;
            }
            pushed[i] = true;
            pp[i] = (pp[qi].0 + x, pp[qi].1 + y);
            qq.push_back(i);
        }
    }
    for (px, py) in pp {
        if (px, py) == (!0, !0) {
            println!("undecidable");
        } else {
            println!("{px} {py}")
        }
    }
}
