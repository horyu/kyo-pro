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
    // l.sqrt() 以下の素数列挙
    let mut primes = vec![];
    {
        let sqrt = l.sqrt();
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
    let mut hm = HashMap::new();
    for p in primes {
        let mut cnt = 0;
        let mut i = p;
        while i < l {
            i = i.saturating_mul(p);
            cnt += 1;
        }
        hm.insert(p, cnt - 1);
    }
    // dbg!(&hm.len());
    let mut rs = 0;
    for mut i in l..=r {
        let mut tmp = vec![];
        for (&k, &v) in hm.iter() {
            let mut cnt = 0;
            while i.is_multiple_of(k) {
                i /= k;
                cnt += 1;
            }
            if v < cnt {
                tmp.push((k, cnt));
            }
        }
        if 1 < i && i < l {
            // 篩から外れた素数
            hm.insert(i, 1);
            i = 1;
        }
        if 1 < i {
            tmp.push((i, 1));
        }
        if !tmp.is_empty() {
            rs += 1;
        }
        for (k, v) in tmp {
            hm.insert(k, v);
        }
    }
    println!("{rs}");
}
