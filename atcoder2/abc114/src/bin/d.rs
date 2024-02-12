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
    };
    // https://blog.hamayanhamayan.com/entry/2018/12/02/233028
    // 75 = 3*5*5
    let mut counter = counter::Counter::<usize, usize>::new();
    for i in 2..=n {
        for (k, c) in factors(i) {
            counter[&k] += c;
        }
    }
    let mut rs = 0usize;
    // 75
    for a in 2..=n {
        if 74 <= counter[&a] {
            rs += 1;
        }
    }
    // 3*25
    for a in 2..=n {
        for b in 2..=n {
            if a != b && 2 <= counter[&a] && 24 <= counter[&b] {
                rs += 1;
            }
        }
    }
    // 5*15
    for a in 2..=n {
        for b in 2..=n {
            if a != b && 4 <= counter[&a] && 14 <= counter[&b] {
                rs += 1;
            }
        }
    }
    // 3*5*5
    for a in 2..=n {
        for b in 2..=n {
            for c in (b + 1)..=n {
                if a != b && a != c && 2 <= counter[&a] && 4 <= counter[&b] && 4 <= counter[&c] {
                    rs += 1;
                }
            }
        }
    }

    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
    };
    // 75 = 3*5*5
    let mut counter = counter::Counter::<usize, usize>::new();
    for i in 2..=n {
        for (k, c) in factors(i) {
            counter[&k] += c;
        }
    }
    let cc = counter
        .into_map()
        .values()
        .copied()
        .filter(|&c| 2 <= c)
        .sorted_unstable()
        .collect_vec();
    let mut rs = 0usize;
    for dd in [vec![75], vec![3, 25], vec![5, 15], vec![3, 5, 5]] {
        let mut tmp = 0;
        for cc in cc.iter().copied().permutations(dd.len()) {
            if izip!(&dd, &cc).all(|(&d, &c)| d <= c + 1) {
                tmp += 1;
            }
        }
        rs += tmp
            / dd.iter()
                .copied()
                .counts()
                .values()
                .copied()
                .map(|i| (1..=i).product::<usize>())
                .product::<usize>();
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
