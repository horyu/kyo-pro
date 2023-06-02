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
        n: usize,
    };
    let mut hs = HashSet::new();
    for k in divisors(n) {
        if k == 1 {
            continue;
        }
        let mut m = n;
        while m % k == 0 {
            m /= k;
        }
        if m % k == 1 {
            hs.insert(k);
        }
    }
    for k in divisors(n - 1) {
        if k == 1 {
            continue;
        }
        hs.insert(k);
    }

    let rs = hs.len();
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
