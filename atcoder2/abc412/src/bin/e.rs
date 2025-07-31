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

fn main() {
    input! {
        l: usize,
        r: usize,
    };
    if l == r {
        println!("1");
        return;
    }
    // r.sqrt() 以下の素数列挙
    let mut primes = vec![];
    {
        let sqrt = r.sqrt();
        let mut is_prime = vec![true; sqrt + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        if 2 <= sqrt {
            primes.push(2);
            // for i in (4..=sqrt).step_by(2) {
            //     is_prime[i] = false;
            // }
        }
        for i in (3..=sqrt).step_by(2) {
            if is_prime[i] {
                primes.push(i);
                for j in ((i * i)..=sqrt).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }
    }
    // dbg!(primes.len());
    let mut vv = vec![true; r - l + 1];
    for p in primes.iter().copied() {
        for i in ((l.div_ceil(p) * p)..=r).step_by(p) {
            vv[i - l] = false;
        }
    }
    for p in primes.iter().copied() {
        let mut x = p;
        while x < l {
            x = x.saturating_mul(p);
        }
        while x <= r {
            vv[x - l] = true;
            x = x.saturating_mul(p);
        }
    }
    vv[0] = true;
    let rs = vv.iter().filter(|&&x| x).count();
    println!("{rs}");
}
