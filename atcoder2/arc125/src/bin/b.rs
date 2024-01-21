#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: u128,
    };
    // https://ichinoseac.hatenablog.com/entry/2021/08/24/101902
    let mut rs = ModInt998244353::default();
    for k in 1..=(n.sqrt()) {
        rs += (k.pow(2) + n) / (2 * k) - k + 1
    }
    println!("{rs}");
}
