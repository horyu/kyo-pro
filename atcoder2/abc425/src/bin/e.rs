#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        t: usize,
        m: usize,
    };
    let mut memo = Vec::with_capacity(5001);
    memo.push(counter::Counter::<usize>::new());
    for i in 1..=5000 {
        let mut counter = memo.last().unwrap().clone();
        for (k, v) in factors(i) {
            counter[&k] += v;
        }
        memo.push(counter);
    }
    let mfactors = factors(m);
    let mfactors_opt = if mfactors.keys().any(|&k| 5000 < k) {
        None
    } else {
        Some(mfactors)
    };

    let mut rs = String::new();
    for _ in 0..t {
        input! { n: usize, cc: [usize; n] };
        if n == 1 {
            rs.push_str("1\n");
            continue;
        }
        let sum = cc.iter().sum::<usize>();
        let mut counter = memo[sum].clone();
        for c in cc {
            for (k, v) in memo[c].iter() {
                counter[k] -= v;
                if counter[k] == 0 {
                    counter.remove(k);
                }
            }
        }
        if let Some(mfactors) = &mfactors_opt {
            if mfactors.len() <= counter.len()
                && mfactors
                    .iter()
                    .all(|(k, v)| v <= counter.get(k).unwrap_or(&0))
            {
                rs.push_str(&format!("0\n"));
                continue;
            }
        }
        let v = counter
            .into_map()
            .into_iter()
            .fold(1usize, |acc, (k, v)| acc * mod_pow(k, v, m) % m);
        rs.push_str(&format!("{v}\n"));
    }
    print!("{rs}");
}

fn factors(mut n: usize) -> HashMap<usize, usize> {
    let mut hm = HashMap::new();
    if n <= 1 {
        return hm;
    }
    while n.is_multiple_of(2) {
        n /= 2;
        *hm.entry(2).or_insert(0) += 1;
    }
    let mut i = 3;
    while i * i <= n {
        while n.is_multiple_of(i) {
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

fn mod_pow(mut x: usize, mut n: usize, m: usize) -> usize {
    x %= m;
    let mut ans = 1;
    while n != 0 {
        if n.is_odd() {
            ans = ans * x % m;
        }
        x = x * x % m;
        n >>= 1;
    }
    ans
}
