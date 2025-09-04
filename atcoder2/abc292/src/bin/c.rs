#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
    let mut rs = 0usize;
    for ab in 1..n {
        let cd = n - ab;
        // 約数の個数が分かればそのかけ合わせ
        // eprintln!("{ab}:{}", divisors(ab).iter().join(" "));
        // eprintln!("{cd}:{}", divisors(cd).iter().join(" "));
        let ab_size = divisors(ab).len();
        let cd_size = divisors(cd).len();
        rs += (ab_size / 2 * 2 + ab_size.is_odd() as usize)
            * (cd_size / 2 * 2 + cd_size.is_odd() as usize);
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
    // divisors.sort_unstable();
    divisors
}
