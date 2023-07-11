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
        n: usize,
        a: usize,
        b: usize,
        c: usize,
    };
    let mut rs = !0usize;
    let max = 9999;
    for i in 0..=max {
        let ai = a * i;
        if n < ai {
            break;
        }
        for j in 0..=(max - i) {
            let aibj = ai + b * j;
            if n < aibj {
                break;
            }
            let k = (n - aibj) / c;
            if i + j + k <= max && aibj + c * k == n {
                rs = rs.min(i + j + k);
            }
        }
    }
    println!("{rs}");
}
