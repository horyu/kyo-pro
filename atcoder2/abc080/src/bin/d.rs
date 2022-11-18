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
        c: usize,
        mut ssttcc: [(usize, usize, Usize1); n],
    };
    ssttcc.sort_unstable_by_key(|stc| stc.1);
    // dbg!(ssttcc);
    let mut aabb: Vec<Vec<(usize, usize)>> = vec![vec![]; c];
    for (s, t, c) in ssttcc {
        if let Some(ab) = aabb[c].last_mut() {
            if ab.1 == s {
                ab.1 = t;
                continue;
            }
        }
        aabb[c].push((s, t));
    }
    let size = 2e5 as usize + 10;
    let mut vv = vec![0isize; size];
    for (c, aabb) in aabb.into_iter().enumerate() {
        for (a, b) in aabb {
            // println!("{c} {a}-{b}");
            vv[2 * a - 1] += 1;
            vv[2 * b] -= 1;
        }
    }
    for i in 1..size {
        vv[i] += vv[i - 1];
    }
    let rs = vv.into_iter().max().unwrap();
    println!("{rs}");
}
