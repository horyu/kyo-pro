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
        dd: [usize; k],
        q: usize,
    };
    let mut mat = vec![vec![!0usize; n]; n];
    for i in 0..n {
        mat[i][i] = 0;
    }
    for (a, b, c) in aabbcc {
        if c < mat[a][b] {
            mat[a][b] = c;
            mat[b][a] = c;
        }
    }
    for di in dd.iter().copied() {
        for dj in dd.iter().copied() {
            if t < mat[di][dj] {
                mat[di][dj] = t;
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let tmp = mat[i][k].saturating_add(mat[k][j]);
                if tmp < mat[i][j] {
                    mat[i][j] = tmp;
                }
            }
        }
    }
    let mut rs = 0;
    for (i, ma) in mat.iter().enumerate() {
        // for (j, m) in ma.iter().copied().enumerate() {
        //     if m != !0 {
        //         rs += m;
        //         eprintln!("{i}-{j}: {m}");
        //     }
        // }
        for m in ma.iter().copied() {
            if m != !0 {
                rs += m;
            }
        }
    }
    let mut dd = HashSet::<usize>::from_iter(dd);
    dbg!(rs);
    for _ in 0..q {
        input! {t: usize};
        if t == 1 {
            input! {x: Usize1, y: Usize1, c: usize};
            if mat[x][y] <= c {
                continue;
            }
            if mat[x][y] == !0 {
                rs += 2 * c;
            } else {
                rs -= 2 * (mat[x][y] - c);
            }
            mat[x][y] = c;
            mat[y][x] = c;
            for i in 0..n {
                if i == x || i == y {
                    continue;
                }
                for j in (i + 1)..n {
                    if j == x || j == y {
                        continue;
                    }
                    let min = (mat[i][x].saturating_add(c).saturating_add(mat[y][j]))
                        .min(mat[i][y].saturating_add(c).saturating_add(mat[x][j]));
                    if min < mat[i][j] {
                        if mat[i][j] == !0 {
                            rs += 2 * min;
                        } else {
                            rs -= 2 * (mat[i][j] - min);
                        }
                        mat[i][j] = min;
                        mat[j][i] = min;
                    }
                }
            }
        } else if t == 2 {
            input! {x: Usize1};
            if dd.contains(&x) {
                continue;
            }
            for d in dd.iter().copied() {
                if mat[x][d] <= t {
                    continue;
                }
                if mat[x][d] == !0 {
                    rs += 2 * t;
                } else {
                    rs -= 2 * (mat[x][d] - t);
                }
                mat[x][d] = t;
                mat[d][x] = t;

            }
            dd.insert(x);
        } else {
            println!("{rs}");
        }
    }
}
