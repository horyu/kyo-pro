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
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        aa: [usize; n],
    };
    let ss = Some(0)
        .iter()
        .chain(aa.iter())
        .cumsum::<usize>()
        .collect_vec();
    for x in 0..(n - 2) {
        let y = match &ss[x..].binary_search(&(ss[x] + p)) {
            Ok(i) => x + i,
            Err(_) => continue,
        };
        let z = match &ss[y..].binary_search(&(ss[y] + q)) {
            Ok(i) => y + i,
            Err(_) => continue,
        };
        // dbg!(y, z, &ss[z..].binary_search(&(ss[z] + r)));
        let w = match &ss[z..].binary_search(&(ss[z] + r)) {
            Ok(i) => z + i,
            Err(_) => continue,
        };
        // eprintln!("{x} {y} {z} {w}");
        println!("Yes");
        return;
    }
    println!("No");
}
