#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
#![feature(hash_drain_filter)]
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
    // 32400 = 2^4 * 3^4 * 5^2
    // 75 = 3^1 * 5^2
    // 3,5,15,25,75
    let mut hm = HashMap::new();
    for i in 1..=n {
        for (k, v) in factors(i) {
            let e = hm.entry(k).or_insert(0usize);
            *e += v;
        }
    }
    let vv = hm
        .into_values()
        .filter(|&v| 1 < v)
        .sorted_unstable()
        .rev()
        .collect_vec();
    let len = vv.len();
    let mut rs = 0usize;
    for i in 0..len {
        if 74 <= vv[i] {
            rs += 1;
        }
        for j in 0..len {
            if i != j {
                if 24 <= vv[i] && 2 <= vv[j] {
                    rs += 1;
                }
                if 14 <= vv[i] && 4 <= vv[j] {
                    rs += 1;
                }

                for k in (j + 1)..len {
                    if i != k && 2 <= vv[i] && 4 <= vv[j] && 4 <= vv[k] {
                        rs += 1;
                    }
                }
            }
        }
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
