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
        mut n: usize,
        cc: [usize; 9],
    };
    let min_c = cc.iter().copied().min().unwrap();
    let size = n / min_c;

    let mut vv = vec![];
    for i in 0..size {
        for (j, c) in cc.iter().copied().enumerate().rev() {
            if min_c * size.saturating_sub(1 + i) + c <= n {
                n -= c;
                vv.push(j + 1);
                break;
            }
        }
    }
    let rs = vv.iter().join("");
    println!("{rs}");
}
