#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
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
        k: usize,
        m: usize,
    };
    // rs = m^(k^n) % p
    let p = 998244353;
    if m % p == 0 {
        println!("0");
    } else {
        // P は素数なのでフェルマーの小定理より
        // m^(p−1) ≡ 1 (mod p)
        // k^n = q*(p-1) + r とすると
        // m^(k^n) ≡ m^(q*(p-1) + r) ≡ m^(q*(p-1)) * m^r ≡ 1^q * m^r (mod p)
        let r = mod_pow(k, n, p - 1);
        let rs = mod_pow(m, r, p);
        println!("{rs}");
    }
}

// https://blog.spiralray.net/cp/modulo#i-8
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
