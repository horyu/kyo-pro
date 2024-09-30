#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
    };
    // p1(x) = x回の移動の後に1にいる確立
    // pn(x) = x回の移動の後に(2~n)にいる確立
    let r = ModInt998244353::new(1) / n;
    let rr = r * r;
    // p0(x+1) =     _ * p0(x) + 2*(n-1)*r^2 * p1(x)
    // p1(x+1) = 2*r^2 * p0(x) +           _ * p1(x)
    let b = ModInt998244353::new(n - 1) * 2 * rr;
    let a = ModInt998244353::new(1) - b;
    let c = ModInt998244353::new(2) * rr;
    let d = ModInt998244353::new(1) - c;
    let mut p1 = ModInt998244353::new(1);
    let mut pn = ModInt998244353::new(0);
    for _ in 0..k {
        (p1, pn) = (a * p1 + b * pn, c * p1 + d * pn);
    }
    // 位置にいる期待値
    let rs = p1 * 1 + pn * (2 + n) * (n - 1) / 2;
    println!("{rs}");
}
