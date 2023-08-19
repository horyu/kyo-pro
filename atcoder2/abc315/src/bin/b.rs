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
        m: usize,
        dd: [usize; m],
    };
    let sum = dd.iter().sum::<usize>();
    let mut cnt = 0;
    for (i, d) in dd.into_iter().enumerate() {
        for j in 1..=d {
            cnt += 1;
            if cnt == (sum + 1) / 2 {
                println!("{} {}", i + 1, j);
                return;
            }
        }
    }
}
