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
        q: usize,
        aa: [usize; q],
    };
    // p, qを 素因数としたとき n = p^2x * q^2y を満たす a 以下の最大の数を求める
    // a <= 1e12 であるため p*q <= 1e6 を全探索する
    let primes = sieve_of_eratosthenes(1e6 as usize)
        .into_iter()
        .positions(|tf| tf)
        .collect_vec();
    let mut vv = vec![];
    for (i, a) in primes.iter().copied().enumerate() {
        for b in primes[(i + 1)..].iter().copied() {
            let mut aa = a;
            while aa * b <= 1e6 as usize {
                vv.push(aa * b);
                let mut bb = b * b;
                while aa * bb <= 1e6 as usize {
                    vv.push(aa * bb);
                    bb *= b;
                }
                aa *= a;
            }
            if aa == a {
                break;
            }
        }
    }
    vv.sort_unstable();
    for a in aa {
        let sqrt = a.sqrt();
        let idx = vv.binary_search(&sqrt).unwrap_or_else(|x| x - 1);
        let rs = vv[idx].pow(2);
        println!("{rs}");
    }
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
