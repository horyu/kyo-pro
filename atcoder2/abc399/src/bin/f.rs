#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::{Mod998244353, ModInt998244353};
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
        k: u64,
        aa: [u32; n],
    };
    // https://atcoder.jp/contests/abc399/editorial/12561
    let mut ss = vec![ModInt998244353::default(); n + 1];
    for (i, a) in aa.into_iter().enumerate() {
        ss[i + 1] = ss[i] + a;
    }
    let mut rs = ModInt998244353::default();
    for kk in 0..=k {
        let mut c = ModInt998244353::new(1);
        for i in 1..=k {
            c *= i;
        }
        for i in 1..=kk {
            c /= i;
        }
        for i in 1..=(k - kk) {
            c /= i;
        }
        let mut s = ModInt998244353::default();
        for i in 0..=n {
            rs += ss[i].pow(k - kk) * s * c;
            s += (-ss[i]).pow(kk);
        }
    }
    println!("{rs}");
}
