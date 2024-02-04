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
        n: usize,
        x: usize,
        mut aa: [usize; n - 1],
    };
    aa.sort_unstable();
    let sum = aa.iter().sum::<usize>();
    for score in 0..=100 {
        let mut tmp = sum + score;
        if score <= aa[0] {
            tmp -= score + aa[n - 2];
        } else if aa[n - 2] <= score {
            tmp -= aa[0] + score;
        } else {
            tmp -= aa[0] + aa[n - 2];
        }
        if x <= tmp {
            println!("{score}");
            return;
        }
    }
    println!("-1");
}
