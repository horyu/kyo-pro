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
        n: usize,
    };
    // 0123: RDLU
    let mut dir = 0;
    let mut mat = vec![vec![0; n]; n];
    mat[n / 2][n / 2] = !0;
    let mut crr = 1;
    let mut x = 0;
    let mut y = 0;
    while crr < n.pow(2) {
        // eprintln!("{x} {y} {dir} {crr}");
        mat[x][y] = crr;
        crr += 1;
        match dir {
            0 => {
                if y + 1 < n && mat[x][y + 1] == 0 {
                    y += 1;
                } else {
                    dir = 1;
                    x += 1;
                }
            }
            1 => {
                if x + 1 < n && mat[x + 1][y] == 0 {
                    x += 1;
                } else {
                    dir = 2;
                    y -= 1;
                }
            }
            2 => {
                if 0 < y && mat[x][y - 1] == 0 {
                    y -= 1;
                } else {
                    dir = 3;
                    x -= 1;
                }
            }
            3 => {
                if 0 < x && mat[x - 1][y] == 0 {
                    x -= 1;
                } else {
                    dir = 0;
                    y += 1;
                }
            }
            _ => unreachable!(),
        }
    }
    for row in mat {
        let s = row
            .into_iter()
            .map(|v| {
                if v == !0 {
                    'T'.to_string()
                } else {
                    v.to_string()
                }
            })
            .join(" ");
        println!("{s}");
    }
}
