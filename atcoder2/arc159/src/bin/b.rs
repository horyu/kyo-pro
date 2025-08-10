#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        a: usize,
        b: usize,
    };
    // let a = 10usize.pow(12) - 3;
    // let b = a - 100;
    let (mut a, mut b) = if a < b { (a, b) } else { (b, a) };
    dbg!(b - a);
    let mut dd = divisors(b - a);
    eprintln!("{dd:?}");
    let d_max = dd.pop().unwrap_or(1);
    let mut rs = 0usize;
    // let mut counter = counter::Counter::<_>::new();
    while 0 < a {
        if a % d_max == 0 && b % d_max == 0 {
            rs += a / d_max;
            break;
        }
        rs += 1;
        let g = dd.iter().copied().rfind(|&d| a % d == 0 && b % d == 0).unwrap_or(1);
        // assert!(g == a.gcd(&b));
        // if counter[&g] == 0 {
        //     eprintln!("{a} {b} : {g} {counter:?}");
        // }
        // counter[&g] += 1;
        a -= g;
        b -= g;
    }
    // dbg!(counter);
    dbg!(a, b);
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
