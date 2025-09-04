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
        mut aaa: [Chars; h],
    };
    let mut qq = VecDeque::new();
    let mut s = (0, 0);
    let mut g = (0, 0);
    for i in 0..h {
        for j in 0..w {
            match aaa[i][j] {
                'S' => s = (i, j),
                'G' => g = (i, j),
                '^' => {
                    for ii in (0..i).rev() {
                        if matches!(aaa[ii][j], '.' | '!') {
                            aaa[ii][j] = '!';
                        } else {
                            break;
                        }
                    }
                }
                'v' => {
                    for ii in (i + 1)..h {
                        if matches!(aaa[ii][j], '.' | '!') {
                            aaa[ii][j] = '!';
                        } else {
                            break;
                        }
                    }
                }
                '>' => {
                    for jj in (j + 1)..w {
                        if matches!(aaa[i][jj], '.' | '!') {
                            aaa[i][jj] = '!';
                        } else {
                            break;
                        }
                    }
                }
                '<' => {
                    for jj in (0..j).rev() {
                        if matches!(aaa[i][jj], '.' | '!') {
                            aaa[i][jj] = '!';
                        } else {
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    // for aa in &aaa {
    //     println!("{}", aa.iter().join(""));
    // }
    let udlr = |i: usize, j: usize| -> Vec<(usize, usize)> {
        let mut vv = vec![];
        if 0 < i {
            vv.push((i - 1, j));
        }
        if i < h - 1 {
            vv.push((i + 1, j));
        }
        if 0 < j {
            vv.push((i, j - 1));
        }
        if j < w - 1 {
            vv.push((i, j + 1));
        }
        vv.retain(|&(vi, vj)| matches!(aaa[vi][vj], '.' | 'G'));
        vv
    };

    let mut pushed = vec![vec![false; w]; h];
    pushed[s.0][s.1] = true;
    qq.push_back((s.0, s.1, 0));
    while let Some((qi, qj, d)) = qq.pop_front() {
        if (qi, qj) == g {
            println!("{d}");
            return;
        }

        for (i, j) in udlr(qi, qj) {
            if !pushed[i][j] {
                pushed[i][j] = true;
                qq.push_back((i, j, d + 1));
            }
        }
    }
    // for pp in pushed {
    //     println!("{}", pp.into_iter().map(|b| if b { "T" } else { "F" }).join(""));
    // }
    println!("-1");
}
