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
        m: usize,
        sss: [Chars; n],
    };
    let ttt = [
        "###.?????".chars().collect_vec(),
        "###.?????".chars().collect_vec(),
        "###.?????".chars().collect_vec(),
        "....?????".chars().collect_vec(),
        "?????????".chars().collect_vec(),
        "?????....".chars().collect_vec(),
        "?????.###".chars().collect_vec(),
        "?????.###".chars().collect_vec(),
        "?????.###".chars().collect_vec(),
    ];
    for i in 0..(n - 8) {
        for j in 0..(m - 8) {
            if ttt.iter().enumerate().all(|(ti, tt)| {
                tt.iter()
                    .copied()
                    .enumerate()
                    .all(|(tj, t)| t == '?' || t == sss[i + ti][j + tj])
            }) {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
    // println!("{rs}");
}
