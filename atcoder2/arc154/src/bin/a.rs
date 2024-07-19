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
        n: usize,
        a: Bytes,
        b: Bytes,
    };
    let mut p = ModInt998244353::default();
    let mut q = ModInt998244353::default();
    for (x, y) in izip!(a, b) {
        p = p * 10 + (x.min(y) - b'0');
        q = q * 10 + (x.max(y) - b'0');
    }
    let rs = p * q;
    println!("{rs}");
}
