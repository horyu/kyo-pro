#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aaa: [[usize; n]; n],
        m: usize,
        xxyy: [(Usize1, Usize1); m]
    };
    let mut ng = vec![vec![false; n]; n];
    for (x, y) in xxyy {
        ng[x][y] = true;
        ng[y][x] = true;
    }
    let min_opt = (0..n)
        .permutations(n)
        .filter_map(|ii| {
            for (&ix, &iy) in ii.iter().tuple_windows() {
                if ng[ix][iy] {
                    return None;
                }
            }
            // eprintln!("[{}] {}", ii.iter().join(","), ii.iter().enumerate().map(|(j, &i)| aaa[j][i]).join(" "));
            Some(
                ii.into_iter()
                    .enumerate()
                    .fold(0usize, |acc, (j, i)| acc + aaa[i][j]),
            )
        })
        .min();
    if let Some(rs) = min_opt {
        println!("{rs}");
    } else {
        println!("-1");
    }
}
