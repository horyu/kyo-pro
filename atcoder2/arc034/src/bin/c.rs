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
        a: usize,
        b: usize,
    };
    let mut counter = counter::Counter::<usize>::new();
    for i in (b + 1)..=a {
        for (k, v) in factors(i) {
            counter[&k] += v;
        }
    }
    let mut rs = ModInt1000000007::new(1);
    for (_, v) in counter {
        rs *= v + 1;
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
