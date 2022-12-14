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
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        aabb: [(Usize1, Usize1); n],
        ccdd: [(Usize1, Usize1); m],
    };
    // 縦横をにチェックしたか
    let mut g = vec![vec![[false; 2]; w]; h];
    let mut blocks = vec![vec![false; w]; h];
    for &(c, d) in &ccdd {
        blocks[c][d] = true;
    }
    for &(a, b) in &aabb {
        g[a][b].fill(true);
        for i in (0..=(a.saturating_sub(1))).rev() {
            if blocks[i][b] || g[i][b][0] {
                break;
            }
            g[i][b][0] = true;
        }
        for i in (a + 1)..h {
            if blocks[i][b] || g[i][b][0] {
                break;
            }
            g[i][b][0] = true;
        }
        for j in (0..=(b.saturating_sub(1))).rev() {
            if blocks[a][j] || g[a][j][1] {
                break;
            }
            g[a][j][1] = true;
        }
        for j in (b + 1)..w {
            if blocks[a][j] || g[a][j][1] {
                break;
            }
            g[a][j][1] = true;
        }
    }
    // let mut s = vec![vec!['F'; w]; h];
    // for &(c, d) in &ccdd {
    //     s[c][d] = 'B';
    // }
    // for &(a, b) in &aabb {
    //     s[a][b] = 'L';
    // }
    // for i in 0..h {
    //     for j in 0..w {
    //         if s[i][j] == 'F' && (g[i][j][0] || g[i][j][0]) {
    //             s[i][j] = 'T';
    //         }
    //     }
    // }
    // for s in s {
    //     eprintln!("{}", s.iter().join(""));
    // }
    let rs = g
        .into_iter()
        .flatten()
        .fold(0, |acc, arr| acc + (arr[0] || arr[1]) as usize);
    println!("{rs}");
}
