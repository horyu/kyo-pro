#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        aa: [usize; n],
    };
    let mut hm = HashMap::new();
    for a in aa.iter().copied() {
        for (k, v) in factors(a) {
            let e = hm.entry(k).or_insert(0);
            *e = v.max(*e);
        }
    }
    let lcm = hm
        .into_iter()
        .fold(ModInt1000000007::new(1), |m, (k, v)| m * k.pow(v as u32));
    let mut rs = ModInt1000000007::new(0);
    for a in aa.iter().copied() {
        rs += lcm / a;
    }
    println!("{rs}");
}

fn factors(mut n: usize) -> HashMap<usize, usize> {
    let mut hm = HashMap::new();
    if n <= 1 {
        return hm;
    }
    while n % 2 == 0 {
        n /= 2;
        *hm.entry(2).or_insert(0) += 1;
    }
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            n /= i;
            *hm.entry(i).or_insert(0) += 1;
        }
        i += 2;
    }
    if n != 1 {
        hm.insert(n, 1);
    }
    hm
}
