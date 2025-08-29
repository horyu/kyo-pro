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
        mut n: usize,
        m: usize,
        mut aabb: [(usize, usize); m],
    };
    // b/a を昇順にソート
    aabb.sort_unstable_by(|(a1, b1), (a2, b2)| (b1 * a2).cmp(&(b2 * a1)));
    for (a,b) in aabb.iter().copied() {
        eprintln!("{a} {b} : {}", b as f64 / a as f64);
    }
    let mut rs = 0usize;
    while let Some((a, b)) = aabb.last().copied() {
        if n < a {
            aabb.pop();
            continue;
        }
        // n -> n-1*(a-b) -> n-2*(a-b) -> ...
        // 初項 n, 公差 -(a-b), 項数 k=(n-a)/(a-b)+1
        let d = a - b;
        let k = (n - a) / d + 1;
        rs += k;
        n -= k * d;
        aabb.pop();
        eprintln!("[{n}] ({a},{b}) {d}x{k}={}", k * d);
    }
    println!("{rs}");
}
