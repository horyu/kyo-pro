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
        sss: [Chars; h],
    };
    let u = (0..h).find(|&i| sss[i].contains(&'#')).unwrap();
    let d = (0..h).rfind(|&i| sss[i].contains(&'#')).unwrap();
    let l = (0..w).find(|&j| (0..h).any(|i| sss[i][j] == '#')).unwrap();
    let r = (0..w).rfind(|&j| (0..h).any(|i| sss[i][j] == '#')).unwrap();
    for i in u..=d {
        for j in l..=r {
            if sss[i][j] == '.' {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
