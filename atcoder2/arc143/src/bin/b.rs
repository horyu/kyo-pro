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
    };
    // https://qiita.com/karutetto332/items/25c4c23f55681a786154
    // NN! - NN * (NN)C(2N-1) * ((N-1)!)^2 * ((N-1)^2)!
    // A     B    C             D            E
    let f = |k: usize| (1..=k).fold(ModInt998244353::new(1), |acc, i| acc * i);

    let a = f(n * n);
    let b = ModInt998244353::new(n * n);
    let c = f(n * n) / f(2 * n - 1) / f(n * n - 2 * n + 1);
    let d = f(n - 1).pow(2);
    let e = f((n - 1).pow(2));

    let rs = a - b * c * d * e;
    println!("{rs}");
}
