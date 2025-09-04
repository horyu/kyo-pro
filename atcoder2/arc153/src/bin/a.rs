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
        n: Usize1,
    };
    // AABCDDEFE
    // F,E,D,C,B,A
    let mut arr = [0, 0, 0, 0, 0, 1];
    for i in 0..n {
        arr[0] += 1;
        for i in 0..5 {
            if arr[i] == 10 {
                arr[i] = 0;
                arr[i + 1] += 1;
            }
        }
    }
    let [f, e, d, c, b, a] = arr;
    let rs = format!("{a}{a}{b}{c}{d}{d}{e}{f}{e}");
    println!("{rs}");
}
