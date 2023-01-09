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
        n: usize,
        ww: [usize; n],
        bb: [usize; n],
    };
    const MAX: usize = (1 + 50) * 50 / 2 + 50;
    let mut grundy = vec![vec![0; MAX + 50]; 50 + 1];
    for i in 0..=50 {
        for j in 0..=MAX {
            let mut mex = vec![0; MAX + 50];
            if 1 <= i {
                mex[grundy[i - 1][j + 1]] = 1;
            }
            if 2 <= j {
                for k in 1..=(j / 2) {
                    mex[grundy[i][j - k]] = 1;
                }
            }
            for k in 0..=MAX {
                if mex[k] == 0 {
                    grundy[i][j] = k;
                    break;
                }
            }
        }
    }

    let sum_xor = izip!(ww, bb)
        .map(|(w, b)| grundy[w][b])
        .fold(0, |acc, g| acc ^ g);
    let rs = ["Second", "First"][(sum_xor != 0) as usize];
    println!("{rs}");
}
