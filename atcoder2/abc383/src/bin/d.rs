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
    let vv = sieve_of_eratosthenes(1e6 as usize);
    // 正の約数が9個
    // a^8 または a^2 * b^2
    // a^8
    // dbg!(n.nth_root(8));
    // dbg!(n.nth_root(8).pow(8));
    // let mut rs = n.nth_root(8) - 1;
    let mut rs = (2..=n.nth_root(8)).filter(|&i| vv[i]).count();
    // a^2 * b^2
    let primes = vv
        .into_iter()
        .positions(|tf| tf)
        .collect_vec();
    for (i, a) in primes.iter().copied().enumerate() {
        let aa = a * a;
        if n <= aa * aa {
            break;
        }
        rs += primes[(i + 1)..].partition_point(|&b| aa.saturating_mul(b * b) <= n);
    }

    println!("{rs}");
}

// エラトステネスの篩
fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let mut vv = vec![true; n + 1];
    vv[0] = false;
    vv[1] = false;
    for i in 2..=n {
        if vv[i] {
            let mut j = i * 2;
            while j <= n {
                vv[j] = false;
                j += i;
            }
        }
    }
    vv
}
