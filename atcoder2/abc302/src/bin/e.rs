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
        q: usize,
    };
    let mut rs = n;
    let mut g = vec![HashSet::new(); n];
    for _ in 0..q {
        input! {t: usize, v: Usize1};
        if t == 1 {
            input! {u: Usize1};
            rs -= (g[u].is_empty()) as usize + (g[v].is_empty()) as usize;
            g[u].insert(v);
            g[v].insert(u);
        } else {
            rs += (!g[v].is_empty()) as usize;
            let gv = std::mem::take(&mut g[v]);
            for u in gv {
                g[u].remove(&v);
                if g[u].is_empty() {
                    rs += 1;
                }
            }
        }
        println!("{rs}");
    }
}
