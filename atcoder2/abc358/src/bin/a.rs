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
use std::io::Read;

fn main() {
    let mut s = String::with_capacity(32);
    std::io::stdin().read_to_string(&mut s).unwrap();
    let tf = s == "AtCoder Land\n";
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
