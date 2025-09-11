#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        m: usize,
        aabbcc: [(Usize1, Usize1, usize); m],
        k: usize,
        t: usize,
        dd: [Usize1; k],
        q: usize,
    };
    const MAX: usize = 1usize << 60;
    let mut mat = vec![vec![MAX; n + 1]; n + 1];
    for i in 0..=n {
        mat[i][i] = 0;
    }
    for (a, b, c) in aabbcc {
        let c = c * 2;
        if c < mat[a][b] {
            mat[a][b] = c;
            mat[b][a] = c;
        }
    }
    for d in dd.iter().copied() {
        mat[d][n] = t;
        mat[n][d] = t;
    }
    for k in 0..=n {
        for i in 0..=n {
            for j in 0..=n {
                let tmp = mat[i][k] + mat[k][j];
                if tmp < mat[i][j] {
                    mat[i][j] = tmp;
                }
            }
        }
    }
    // for ma in mat.iter() {
    //     eprintln!("{:?}", ma);
    // }
    for _ in 0..q {
        input! {qt: usize};
        if qt == 1 {
            input! {x: Usize1, y: Usize1, c: usize};
            let c = c * 2;
            if mat[x][y] <= c {
                continue;
            }
            mat[x][y] = c;
            mat[y][x] = c;
            for i in 0..=n {
                for j in (i + 1)..=n {
                    let min = (mat[i][x] + c + mat[y][j]).min(mat[i][y] + c + mat[x][j]);
                    if min < mat[i][j] {
                        eprintln!("[1] {i}-{j}: {min} <- {}", mat[i][j]);
                        mat[i][j] = min;
                        mat[j][i] = min;
                    }
                }
            }
        } else if qt == 2 {
            input! {x: Usize1};
            if mat[x][n] <= t {
                continue;
            }
            mat[x][n] = t;
            mat[n][x] = t;
            for i in 0..=n {
                for j in (i + 1)..=n {
                    let min = (mat[i][x] + t + mat[n][j]).min(mat[i][n] + t + mat[x][j]);
                    if min < mat[i][j] {
                        eprintln!("[2] {i}-{j}: {min} <- {}", mat[i][j]);
                        mat[i][j] = min;
                        mat[j][i] = min;
                    }
                }
            }
        } else {
            let mut sum = 0;
            for ma in mat.iter().take(n) {
                for m in ma.iter().copied().take(n) {
                    if m != MAX {
                        sum += m;
                    }
                }
            }
            println!("{}", sum / 2);
        }
    }
}
