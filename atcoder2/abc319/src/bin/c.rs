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
        cc: [isize; 9],
    };
    let bunbo = (1..=9).product::<usize>() as f64;
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
            if !xx.iter().copied().map(|x| cc[x]).all_unique()
                && xx
                    .iter()
                    .copied()
                    .sorted_unstable_by_key(|&x| ii.iter().copied().position(|i| i == x).unwrap())
                    .take(2)
                    .map(|x| cc[x])
                    .all_equal()
            {
                ok = false;
                break;
            }
        }
        if ok {
            bunshi += 1.0;
        }
    }
    let rs = bunshi / bunbo;
    println!("{rs}");
}
