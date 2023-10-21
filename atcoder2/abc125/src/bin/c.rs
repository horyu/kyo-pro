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
        aa: [usize; n],
    };
    let mut counter = counter::Counter::<usize>::new();
    for a in aa[..2].iter().copied() {
        for d in divisors(a) {
            counter[&d] += 1;
        }
    }
    for a in aa[2..].iter().copied() {
        for (k, v) in counter.iter_mut() {
            if a % k == 0 {
                *v += 1;
            }
        }
    }
    let rs = counter
        .into_iter()
        .filter_map(|kv| (n - 1 <= kv.1).then_some(kv.0))
        .max()
        .unwrap_or(1);
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
    // divisors.sort_unstable();
    divisors
}
