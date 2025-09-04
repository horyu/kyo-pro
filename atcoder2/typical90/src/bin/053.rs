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
    let mut stdin =
        proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    // https://x.com/e869120/status/1398827248698810369
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {n: usize};
        if n <= 5 {
            let mut rs = 0;
            for i in 0..n {
                println!("? {}", i + 1);
                input! {a: isize};
                rs = rs.max(a);
            }
            println!("! {rs}");
            continue;
        }
        let m = 1597;
        // 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597,
        let mut aa = vec![None; m];
        aa[0] = Some(-1);
        for i in (n + 1)..m {
            aa[i] = Some(-(i as isize))
        }
        let mut q = |i: usize| {
            if aa[i].is_none() {
                println!("? {i}");
                input! {a: isize};
                aa[i] = Some(a);
            }
            aa[i].unwrap()
        };
        let (mut l, mut r) = (0, m);
        let (mut x, mut y) = (610, 987);
        for _ in 0..14 {
            let ax = q(x);
            let ay = q(y);
            if ax < ay {
                (l, r, x, y) = (x, r, y, r - (y - x));
            } else {
                (l, r, x, y) = (l, y, l + (y - x), x);
            }
        }
        let rs = aa.into_iter().flatten().max().unwrap();
        println!("! {rs}");
    }
}
