#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut ll = VecDeque::new();
    let mut rr = VecDeque::new();
    ll.push_back(0);
    rr.push_back(0);
    for i in 0..n {
        ll.push_back(gcd(*ll.back().unwrap(), aa[i]));
        rr.push_front(gcd(aa[n - 1 - i], *rr.front().unwrap()));
    }
    let rs = (0..n).map(|i| gcd(ll[i], rr[i + 1])).max().unwrap();
    println!("{}", rs);
}

fn gcd(x: usize, y: usize) -> usize {
    if x == 0 {
        y
    } else if y == 0 {
        x
    } else {
        x.gcd(&y)
    }
}

fn main2() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut hm = HashMap::new();
    let (left, right) = aa.split_at(2);
    for &a in left {
        for d in divisors(a) {
            *hm.entry(d).or_insert(0usize) += 1;
        }
    }
    for &a in right {
        for (k, v) in hm.iter_mut() {
            if a % k == 0 {
                *v += 1;
            }
        }
    }
    let mut x = 0;
    let mut y = 0;
    for (k, cnt) in hm {
        if cnt == n {
            x = x.max(k);
        } else if cnt == n - 1 {
            y = y.max(k);
        }
    }
    println!("{}", x.max(y));
}

// 公約数 https://qiita.com/Cassin01/items/9bc63a4bde5526150681
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
    divisors
}
