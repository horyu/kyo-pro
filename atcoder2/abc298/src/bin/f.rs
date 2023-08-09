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
        rrccxx: [(isize, isize, usize); n],
    };
    let mut hm = HashMap::new();
    let mut tate = counter::Counter::<isize>::new();
    let mut yoko = counter::Counter::<isize>::new();
    for (r, c, x) in rrccxx.iter().copied() {
        hm.insert((r, c), x);
        tate[&r] += x;
        yoko[&c] += x;
    }
    let yoko = yoko
        .into_iter()
        .sorted_unstable_by_key(|cx| cx.1)
        .rev()
        .collect_vec();
    let mut rs = 0;
    for (r, rsum) in tate {
        for (c, csum) in yoko.iter().copied() {
            if let Some(x) = hm.get(&(r, c)) {
                rs = rs.max(rsum + csum - x);
            } else {
                rs = rs.max(rsum + csum);
                break;
            }
        }
    }

    println!("{rs}");
}
