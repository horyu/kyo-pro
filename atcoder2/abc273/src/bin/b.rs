#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        mut x: usize,
        k: u32,
    };
    for i in 0..k {
        let pow = 10usize.pow(i);
        let a = x / pow % 10;
        if a < 5 {
            x -= a * pow;
        } else {
            x += (10 - a) * pow
        }
    }
    println!("{x}");
}
