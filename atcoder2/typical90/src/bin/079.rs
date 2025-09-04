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
        h: usize,
        w: usize,
        mut aaa: [[isize; w]; h],
        bbb: [[isize; w]; h],
    };
    let mut cnt = 0;
    for i in 0..(h - 1) {
        for j in 0..(w - 1) {
            let diff = bbb[i][j] - aaa[i][j];
            aaa[i][j] += diff;
            aaa[i][j + 1] += diff;
            aaa[i + 1][j] += diff;
            aaa[i + 1][j + 1] += diff;

            cnt += diff.abs();
        }
    }
    let tf = aaa == bbb;
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
    if tf {
        println!("{cnt}");
    }
}
