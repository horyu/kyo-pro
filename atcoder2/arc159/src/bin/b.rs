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

/*
12345678910 10987654321
===
[src\bin\b.rs:20:5] b - a = 1358024589
16: [1, 3, 9, 27, 127, 381, 1143, 3429, 396041, 1188123, 3564369, 10693107, 50297207, 150891621, 452674863, 1358024589]
10987654321 12345678910 : 1 Counter { map: {}, zero: 0 }
10987654320 12345678909 : 27 Counter { map: {1: 1}, zero: 0 }
// 27 = 3*3*3
10987650999 12345675588 : 3429 Counter { map: {27: 123, 1: 1}, zero: 0 }
// 3429 = 3*3*3*127
10864196712 12222221301 : 1358024589 Counter { map: {3429: 36003, 27: 123, 1: 1}, zero: 0 }
// 1358024589 = 3*3*3*127*396041
[src\bin\b.rs:35:5] counter = Counter {
    map: {
        1: 1,
        27: 123,
        3429: 36003,
        1358024589: 8,
    },
    zero: 0,
}
*/
fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    };
    if a == b {
        println!("1");
        return;
    }
    // https://atcoder.jp/contests/arc159/editorial/6106
    // https://atcoder.jp/contests/arc159/submissions/45734011
    if b < a {
        std::mem::swap(&mut a, &mut b);
    }
    let mut rs = 0usize;
    while 0 < a {
        let g = a.gcd(&b);
        a /= g;
        b /= g;
        if b - a == 1 {
            rs += a;
            break;
        }
        let mut t = !0;
        for d in divisors(b - a).into_iter().skip(1) {
            t = t.min(a % d);
        }
        rs += t;
        a -= t;
        b -= t;
    }
    println!("{rs}");
}

// https://qiita.com/Cassin01/items/9bc63a4bde5526150681
fn divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    for i in 1..=(f64::sqrt(n as f64) + 1e-9) as usize {
        if n.is_multiple_of(i) {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }
    divisors.sort_unstable();
    divisors
}
