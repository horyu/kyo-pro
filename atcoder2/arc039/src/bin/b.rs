#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt1000000007;
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
        k: usize,
    };
    let r = k / n;
    if r == 0 {
        let mut rs = ModInt1000000007::new(1);
        for x in 1..=(k + n - 1) {
            rs *= x;
        }
        for x in chain!(1..=k, 1..=(n - 1)) {
            rs /= x;
        }
        println!("{rs}");
        return;
    }
    // a + b = n
    // a * r + b * (1 + r) = k
    let b = k - r * n;
    let a = n - b;
    // (a + b)!/ a! / b!
    // dbg!(r, a, b);
    let mut rs = ModInt1000000007::new(1);
    for x in 1..=(a + b) {
        rs *= x;
    }
    for x in chain(1..=a, 1..=b) {
        rs /= x;
    }
    println!("{rs}");
}
