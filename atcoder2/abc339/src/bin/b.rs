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
        n: usize,
    };
    let mut dir = 0;
    let ddxx = [-1, 0, 1, 0];
    let ddyy = [0, 1, 0, -1];
    let mut xx = 0;
    let mut yy = 0;
    let mut ccc = vec![vec!['.'; w]; h];
    let h = h as i32;
    let w = w as i32;
    for k in 0..n {
        let x = xx as usize;
        let y = yy as usize;
        if ccc[x][y] == '.' {
            ccc[x][y] = '#';
            dir = (dir + 1) % 4;
        } else {
            ccc[x][y] = '.';
            dir = (dir + 3) % 4;
        }
        xx += ddxx[dir];
        yy += ddyy[dir];
        xx = xx.mod_floor(&h);
        yy = yy.mod_floor(&w);
    }
    for cc in ccc {
        println!("{}", cc.into_iter().join(""));
    }
}
