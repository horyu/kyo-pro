#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        t: usize,
    };
    // https://atcoder.jp/contests/abc240/editorial/3425
    fn sum(l: isize, r: isize, cnt: isize) -> isize {
        (l + r) * cnt / 2
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            xxyy: [(isize, isize); n],
        };
        let mut cv = 0isize;
        let mut cx = 0isize;
        let mut rs = isize::MIN / 4;
        for (x, y) in xxyy {
            let vl = cv + x;
            let vr = cv + x * y;
            if x != 0 && vr <= 0 && 0 <= vl {
                let pt = (-cv) / x;
                rs = rs.max(cx + sum(vl, cv + x * pt, pt));
            }
            rs = rs.max(cx + vl);
            cv = vr;
            cx += sum(vl, vr, y);
            rs = rs.max(cx);
        }
        println!("{rs}");
    }
}
