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
        n: u64,
    };
    let log = n.ilog10();
    // f(4) = 4 * 1e0 + 4 * 1e1 + 4 * 1e2 + 4 * 1e3
    //  = 4 * (1e0 + 1e1 + 1e2 + 1e3)
    // f(12) = 121212121212121212121212
    // = 12* 1e0 + 12 * 1e2 + 12 * 1e4 + ... + 12 * 1e22
    // = 12 * (1e0 + 1e2 + 1e4 + ... + 1e22)

    // 指数部 初項1, 公比r 10^(log10(n) + 1), 項数n の等比数列の和
    // 1 * (r^n - 1) / (r - 1)
    let r = ModInt998244353::new(10usize.pow(log + 1));
    let rs = ModInt998244353::new(n) * ((r.pow(n) - 1) / (r - 1));
    println!("{rs}");
}
