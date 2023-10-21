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
        ccc: [Chars; h],
    };
    let mut ii: HashSet<usize> = (0..h).collect();
    let mut jj: HashSet<usize> = (0..w).collect();
    loop {
        let mut remove_ii = vec![];
        let mut remove_jj = vec![];
        if 2 <= jj.len() {
            remove_ii.extend(
                ii.iter()
                    .filter(|&&i| jj.iter().copied().map(|j| ccc[i][j]).all_equal()),
            );
        }
        if 2 <= ii.len() {
            remove_jj.extend(
                jj.iter()
                    .filter(|&&j| ii.iter().copied().map(|i| ccc[i][j]).all_equal()),
            );
        }
        if remove_ii.is_empty() && remove_jj.is_empty() {
            break;
        }
        for i in remove_ii {
            ii.remove(&i);
        }
        for j in remove_jj {
            jj.remove(&j);
        }
    }
    let rs = ii.len() * jj.len();
    println!("{rs}");
}
