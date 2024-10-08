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
        a: (isize, isize),
        b: (isize, isize),
        n: usize,
        xxyy: [(isize, isize); n]
    };
    let mut cnt = 0;
    for (c, d) in xxyy.iter().copied().cycle().tuple_windows().take(n) {
        if judge(a, b, c, d) {
            cnt += 1;
        }
    }
    let rs = cnt / 2 + 1;
    println!("{rs}");
}

// [2線分の交差判定手法 (2次元) - Qiita](https://qiita.com/zu_rin/items/e04fdec4e3dec6072104)
// 直線に対してもう片方の線分の端点それぞれが逆の領域にあるかどうかで判定
type P = (isize, isize);
fn judge(a: P, b: P, c: P, d: P) -> bool {
    let s = (a.0 - b.0) * (c.1 - a.1) - (a.1 - b.1) * (c.0 - a.0);
    let t = (a.0 - b.0) * (d.1 - a.1) - (a.1 - b.1) * (d.0 - a.0);
    if 0 < s * t {
        return false;
    }
    let s = (c.0 - d.0) * (a.1 - c.1) - (c.1 - d.1) * (a.0 - c.0);
    let t = (c.0 - d.0) * (b.1 - c.1) - (c.1 - d.1) * (b.0 - c.0);
    if 0 < s * t {
        return false;
    }
    true
}
