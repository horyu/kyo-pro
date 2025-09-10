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
    let mut sum = 0;
    for (i, ma) in mat[..n].iter().enumerate() {
        for (j, m) in ma.iter().copied().enumerate().skip(i + 1).take(n - i - 1) {
            if 0 < m && m < MAX {
                sum += 2 * m;
                eprintln!("{i}-{j}: {m}");
            }
        }
    }
    dbg!(sum);
    for _ in 0..q {
        input! {qt: usize};
        if qt == 1 {
            input! {x: Usize1, y: Usize1, c: usize};
            let c = c * 2;
            if mat[x][y] <= c {
                continue;
            }
            if mat[x][y] == MAX {
                sum += 2 * c;
            } else {
                sum -= 2 * (mat[x][y] - c);
            }
            mat[x][y] = c;
            mat[y][x] = c;
            for i in 0..=n {
                for j in (i + 1)..=n {
                    let min = (mat[i][x] + c + mat[y][j]).min(mat[i][y] + c + mat[x][j]);
                    if min < mat[i][j] {
                        if i != n && j != n {
                            if mat[i][j] == MAX {
                                sum += 2 * min;
                            } else {
                                sum -= 2 * (mat[i][j] - min);
                            }
                        }
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
                        if i != n && j != n {
                            if mat[i][j] == MAX {
                                sum += 2 * min;
                            } else {
                                sum -= 2 * (mat[i][j] - min);
                            }
                        }
                        eprintln!("[2] {i}-{j}: {min} <- {}", mat[i][j]);
                        mat[i][j] = min;
                        mat[j][i] = min;
                    }
                }
            }
        } else {
            println!("{}", sum / 2);
        }
    }
}
