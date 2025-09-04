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
        h: usize,
        w: usize,
        m: usize,
        hhww: [(Usize1, Usize1); m],
    };
    let mut hs = HashSet::new();
    let mut y2cnt = vec![0usize; h];
    let mut x2cnt = vec![0usize; w];
    for (h, w) in hhww {
        hs.insert((h, w));
        y2cnt[h] += 1;
        x2cnt[w] += 1;
    }
    let y_max = y2cnt.iter().max().copied().unwrap();
    let x_max = x2cnt.iter().max().copied().unwrap();
    let yy = y2cnt.into_iter().positions(|c| c == y_max).collect_vec();
    let xx = x2cnt.into_iter().positions(|c| c == x_max).collect_vec();
    let mut rs = y_max + x_max;
    for &y in &yy {
        for &x in &xx {
            if !hs.contains(&(y, x)) {
                println!("{rs}");
                return;
            }
        }
    }
    rs -= 1;
    println!("{rs}");
}
