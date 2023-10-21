#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        n: usize,
        hh: usize,
        ww: usize,
        aaa: [[Usize1; w]; h],
    };
    let mut sum = vec![vec![vec![0isize; n]; w + 1]; h + 1];
    for (i, aa) in aaa.iter().enumerate() {
        for (j, &a) in aa.iter().enumerate() {
            for p in 0..n {
                sum[i + 1][j + 1][p] =
                    sum[i + 1][j][p] + sum[i][j + 1][p] - sum[i][j][p] + isize::from(p == a);
            }
        }
    }
    let all = sum[h][w].clone();
    let base = all.iter().filter(|cnt| cnt.is_positive()).count();
    // eprintln!("{}", all.iter().join(" "));
    // dbg!(base);
    for k in 0..=(h - hh) {
        let mut rs = vec![base; w - ww + 1];
        for l in 0..=(w - ww) {
            for p in 0..n {
                if all[p] == 0 {
                    continue;
                }
                if all[p]
                    == sum[k + hh][l + ww][p] + sum[k][l][p] - sum[k + hh][l][p] - sum[k][l + ww][p]
                {
                    rs[l] -= 1;
                }
            }
        }
        println!("{}", rs.iter().join(" "));
    }
    // println!("{rs}");
}
