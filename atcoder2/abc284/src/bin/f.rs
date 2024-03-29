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
        n: usize,
        t: Bytes,
    };
    // https://atcoder.jp/contests/abc284/editorial/5469
    let a = t[..n].iter().copied().collect_vec();
    let b = t[n..].iter().copied().rev().collect_vec();

    let x = chain!(&a, &b).copied().collect_vec();
    let mut za_x = ac_library::z_algorithm_arbitrary(&x);
    za_x.push(0);

    let y = chain!(&b, &a).copied().collect_vec();
    let mut za_y = ac_library::z_algorithm_arbitrary(&y);
    za_y.push(0);
    for i in 0..=n {
        if za_x[2 * n - i] < i {
            continue;
        }
        if za_y[n + i] < n - i {
            continue;
        }
        let rs = chain!(&t[..i], &t[(n + i)..])
            .map(|&b| b as char)
            .collect::<String>();
        println!("{rs}");
        println!("{i}");
        return;
    }
    println!("-1");
}
