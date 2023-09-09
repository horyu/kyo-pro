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
    };
    let mut rs = String::new();
    for i in 0..=n {
        if let Some(j) = (1..=9).find(|&j| n % j == 0 && i % (n / j) == 0) {
            rs.push((b'0' + j as u8) as char);
        } else {
            rs.push('-');
        }
    }
    println!("{rs}");
}
