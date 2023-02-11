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
        a: usize,
        b: usize,
    };
    use num_bigint::ToBigUint;
    let a = a.to_biguint().unwrap();
    let b = b.to_biguint().unwrap();
    let lcm = a.lcm(&b);
    if lcm <= 1e18.to_biguint().unwrap() {
        println!("{lcm}");
    } else {
        println!("Large");
    }
}
