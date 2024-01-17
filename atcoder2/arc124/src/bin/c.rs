#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        aabb: [(usize, usize); n],
    };
    let cc = divisors(aabb[0].0);
    let dd = divisors(aabb[0].1);
    let mut rs = 1;
    for c in cc.iter().copied() {
        for d in dd.iter().copied() {
            if aabb
                .iter()
                .copied()
                .all(|(a, b)| (a % c == 0 && b % d == 0) || (a % d == 0 && b % c == 0))
            {
                rs = rs.max(c.lcm(&d));
            }
        }
    }
    println!("{rs}");
}

// https://qiita.com/Cassin01/items/9bc63a4bde5526150681
fn divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    for i in 1..=(f64::sqrt(n as f64) + 1e-9) as usize {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }
    divisors.sort_unstable();
    divisors
}
