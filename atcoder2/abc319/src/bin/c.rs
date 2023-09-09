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
        ccc: [[isize; 3]; 3],
    };
    let bunbo = (1..=9).fold(1.0, |acc, i| acc * i as f64);
    let mut bunshi = 0.0;
    // dbg!((0..9).permutations(9).count());
    for ii in (0..9).permutations(9) {
        let mut ok = true;
        for xx in [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ] {
            let arr = [
                ccc[xx[0] / 3][xx[0] % 3],
                ccc[xx[1] / 3][xx[1] % 3],
                ccc[xx[2] / 3][xx[2] % 3],
            ];
            if arr[0] == arr[1] || arr[1] == arr[2] || arr[2] == arr[0] {
                // 順番をたしかめる
                let yy = xx
                    .iter()
                    .copied()
                    .sorted_unstable_by_key(|&x| ii.iter().copied().position(|i| i == x).unwrap())
                    .collect_vec();
                if ccc[yy[0] / 3][yy[0] % 3] == ccc[yy[1] / 3][yy[1] % 3] {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            bunshi += 1.0;
        }
    }
    let rs = bunshi / bunbo;
    println!("{rs}");
}
