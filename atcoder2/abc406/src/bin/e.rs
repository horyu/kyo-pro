#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
#![allow(clippy::assign_op_pattern)]
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
        t: usize,
        nnkk: [(usize, usize); t],
    };
    // https://atcoder.jp/contests/abc406/editorial/13057
    // c(n, k) = 0以上n以下の整数のうち popcountがkのものの個数
    let mut c = HashMap::new();
    // s(n, k) = 0以上n以下の整数のうち popcountがkのものの総和
    let mut s = HashMap::new();
    for (n, k) in nnkk {
        let rs = fs(n, k, &mut c, &mut s);
        println!("{rs}");
    }
}

fn fs(
    n: usize,
    k: usize,
    c: &mut HashMap<(usize, usize), ModInt998244353>,
    s: &mut HashMap<(usize, usize), ModInt998244353>,
) -> ModInt998244353 {
    if let Some(&v) = s.get(&(n, k)) {
        return v;
    }
    if n == 0 || k == 0 {
        s.insert((n, k), ModInt998244353::new(0));
        return ModInt998244353::new(0);
    }
    let v = fs(n / 2, k, c, s) * 2 + fs((n - 1) / 2, k - 1, c, s) * 2 + fc((n - 1) / 2, k - 1, c);
    s.insert((n, k), v);
    v
}

fn fc(n: usize, k: usize, c: &mut HashMap<(usize, usize), ModInt998244353>) -> ModInt998244353 {
    if let Some(&v) = c.get(&(n, k)) {
        return v;
    }
    if k == 0 {
        c.insert((n, k), ModInt998244353::new(1));
        return ModInt998244353::new(1);
    }
    if n == 0 {
        c.insert((n, k), ModInt998244353::new(0));
        return ModInt998244353::new(0);
    }
    let v = fc(n / 2, k, c) + fc((n - 1) / 2, k - 1, c);
    c.insert((n, k), v);
    v
}
