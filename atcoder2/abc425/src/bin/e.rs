#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
#![allow(dead_code)]
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

fn main2() {
    input! {
        t: usize,
        m: usize,
    };
    const K: usize = 5010;
    let mut binom = vec![vec![0usize; K]; K];
    binom[0][0] = 1;
    for n in 1..K {
        binom[n][0] = 1;
        for k in 1..=n {
            binom[n][k] = (binom[n - 1][k - 1] + binom[n - 1][k]) % m;
        }
    }
    for _ in 0..t {
        input! { n: usize, cc: [usize; n] };
        let mut rs = 1;
        let mut s = 0;
        for c in cc {
            s += c;
            rs *= binom[s][c];
            rs %= m;
        }
        println!("{rs}");
    }
}

fn main() {
    input! {
        t: usize,
        m: usize,
    };
    // エラトステネスの篩を使ってマップを作る
    let (i2p, p2i) = {
        let mut is_prime = vec![true; 5001];
        let mut i2p = Vec::new();
        let mut p2i = vec![0usize; 5001];
        let mut cnt = 0;
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..=5000 {
            if is_prime[i] {
                p2i[i] = cnt;
                i2p.push(i);
                cnt += 1;
                let mut j = i * i;
                while j <= 5000 {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }
        (i2p, p2i)
    };
    let mut memo = Vec::with_capacity(5001);
    memo.push(vec![0]);
    for i in 1..=5000 {
        let mut vv = memo.last().unwrap().clone();
        if 0 < p2i[i] {
            vv.push(0);
        }
        for (k, v) in factors(i) {
            vv[p2i[k]] += v;
        }
        memo.push(vv);
    }
    let mut rs = String::new();
    for _ in 0..t {
        input! { n: usize, cc: [usize; n] };
        if n == 1 {
            rs.push_str("1\n");
            continue;
        }
        let sum = cc.iter().sum::<usize>();
        let mut xx = memo[sum].clone();
        for c in cc {
            for (x, y) in izip!(&mut xx, &memo[c]) {
                *x -= *y;
            }
        }
        let v = izip!(i2p.iter().copied(), xx.into_iter())
            .fold(1usize, |acc, (p, e)| acc * mod_pow(p, e, m) % m);
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
