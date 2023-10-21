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
        // aa: [usize; n],
        // bb: [usize; n],
        // cc: [usize; n],
        xxx: [[usize; n]; 3],
    };
    const SIZE: usize = 46;
    let mut ccc = [[0usize; SIZE]; 3];
    for (i, xx) in xxx.into_iter().enumerate() {
        for x in xx {
            ccc[i][x % SIZE] += 1;
        }
    }
    let mut rs = 0usize;
    for i in 0..SIZE {
        for j in 0..SIZE {
            let k = (SIZE * 2 - i - j) % SIZE;
            rs += ccc[0][i] * ccc[1][j] * ccc[2][k];
        }
    }
    println!("{rs}");
}
