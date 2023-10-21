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

/*
0	49 49 49 .. 49 49
1	99 48 48 .. 48 48
2	98 98 47 .. 47 47
3	97 97 97 .. 46 46

49	51 51 51 .. 51 0
50	50 50 50 .. 50 50
*/

fn main() {
    input! {
        k: usize,
    };
    let mut vv = [49 + k / 50; 50];
    let amari = k % 50;
    for i in 0..amari {
        vv[i] += 51 - amari;
    }
    for i in amari..50 {
        vv[i] -= amari;
    }
    println!("50\n{}", vv.into_iter().join(" "));
}
