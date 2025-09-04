#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    io::Write,
};

fn main() {
    let mut stdin =
        proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    input! {
        n: usize
    };
    // ある頂点から一番遠い頂点を求め、その頂点から一番遠い頂点までの距離が直径
    let mut hm = HashMap::new();
    for i in 2..=n {
        println!("? 1 {i}");
        input!(
            d: usize
        );
        hm.insert(i, d);
    }
    let i = hm.into_iter().max_by_key(|(_, d)| *d).unwrap().0;
    let mut rs = 0;
    for j in 1..=n {
        if i == j {
            continue;
        }
        println!("? {i} {j}");
        input!(
            d: usize
        );
        rs = rs.max(d);
    }
    println!("! {rs}");
}
