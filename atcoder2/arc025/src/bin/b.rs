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
        ccc: [[usize; w]; h],
    };
    let mut aaa = vec![vec![0usize; w + 1]; h + 1];
    let mut bbb = aaa.clone();
    // 平面累積和
    for i in 0..h {
        for j in 0..w {
            aaa[i + 1][j + 1] = aaa[i + 1][j] + aaa[i][j + 1] - aaa[i][j];
            bbb[i + 1][j + 1] = bbb[i + 1][j] + bbb[i][j + 1] - bbb[i][j];
            if (i + j) % 2 == 0 {
                aaa[i + 1][j + 1] += ccc[i][j];
            } else {
                bbb[i + 1][j + 1] += ccc[i][j];
            }
        }
    }
    let mut rs = 0usize;
    for i in 0..h {
        for j in 0..w {
            for ii in (i + 1)..=h {
                for jj in (j + 1)..=w {
                    let a = aaa[ii][jj] + aaa[i][j] - aaa[i][jj] - aaa[ii][j];
                    let b = bbb[ii][jj] + bbb[i][j] - bbb[i][jj] - bbb[ii][j];
                    if a == b {
                        rs = rs.max((ii - i) * (jj - j));
                    }
                }
            }
        }
    }
    println!("{rs}");
}
