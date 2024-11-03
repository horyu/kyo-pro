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
        n: usize,
        aaa: [[[isize; n]; n]; n],
        q: usize,
        qq: [(Usize1, Usize1, Usize1, Usize1, Usize1, Usize1); q],
    };
    // 3次元累積和
    let mut sss = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                sss[i + 1][j + 1][k + 1] = aaa[i][j][k]
                    + sss[i + 1][j + 1][k]
                    + sss[i + 1][j][k + 1]
                    + sss[i][j + 1][k + 1]
                    - sss[i + 1][j][k]
                    - sss[i][j + 1][k]
                    - sss[i][j][k + 1]
                    + sss[i][j][k];
            }
        }
    }
    for (xl, xr, yl, yr, zl, zr) in qq {
        let rs = sss[xr + 1][yr + 1][zr + 1]
            - sss[xl][yr + 1][zr + 1]
            - sss[xr + 1][yl][zr + 1]
            - sss[xr + 1][yr + 1][zl]
            + sss[xl][yl][zr + 1]
            + sss[xl][yr + 1][zl]
            + sss[xr + 1][yl][zl]
            - sss[xl][yl][zl];
        println!("{rs}");
    }
}
