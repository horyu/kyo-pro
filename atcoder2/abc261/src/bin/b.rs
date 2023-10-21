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
        aaa: [Chars; n],
    };
    for (i, j) in (0..n).tuple_combinations() {
        let ij = aaa[i][j];
        let ji = aaa[j][i];
        let is_invalid = match ij {
            'W' => ji != 'L',
            'L' => ji != 'W',
            'D' => ji != 'D',
            _ => unreachable!(),
        };
        if is_invalid {
            println!("incorrect");
            return;
        }
    }
    println!("correct");
}
