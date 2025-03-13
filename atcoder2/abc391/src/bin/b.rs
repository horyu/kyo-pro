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
        m: usize,
        ss: [Chars; n],
        tt: [Chars; m],
    };
    for a in 0..=(n - m) {
        for b in 0..=(n - m) {
            if tt
                .iter()
                .enumerate()
                .all(|(i, t)| t.iter().enumerate().all(|(j, &c)| ss[a + i][b + j] == c))
            {
                println!("{} {}", a + 1, b + 1);
                return;
            }
        }
    }
}
