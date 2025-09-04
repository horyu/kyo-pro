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
        n: u128,
    };
    const SIZE: usize = 1e6 as usize;
    let mut vv = vec![true; SIZE + 1];
    vv[0] = false;
    vv[1] = false;
    let mut primes = vec![];
    for i in 2..=SIZE {
        if vv[i] {
            primes.push(i as u128);
            let mut j = i * 2;
            while j <= SIZE {
                vv[j] = false;
                j += i;
            }
        }
    }
    let mut rs = 0usize;
    let len = primes.len();
    for i in 0..(len - 2) {
        let aa = primes[i] * primes[i];
        if n <= aa {
            break;
        }
        for j in (i + 1)..(len - 1) {
            let aab = aa * primes[j];
            if n <= aab {
                break;
            }
            // 二分探索
            let mut l = j + 1;
            if n < aab * primes[l] {
                break;
            }
            let mut r = len;
            while l < r {
                let m = (l + r) / 2;
                if aab * primes[m] * primes[m] <= n {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
            rs += l - j - 1;
        }
    }

    println!("{rs}");
}
