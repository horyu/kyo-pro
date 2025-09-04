#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        a: isize,
        b: isize,
        c: isize,
        d: isize,
        e: isize,
        f: isize,
    };
    let mut x = ModInt998244353::from(0);
    x += a;
    x *= b;
    x *= c;
    let mut y = ModInt998244353::from(0);
    y += d;
    y *= e;
    y *= f;
    let rs = x - y;
    println!("{rs}");
}
