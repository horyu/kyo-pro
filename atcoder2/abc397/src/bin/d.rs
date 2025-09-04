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
    input! {
        n: i128,
    };
    // https://atcoder.jp/contests/abc397/editorial/12454
    // d = x - y; y = k とすると x = d + k
    // xxx - yyy = n
    // (d+k)^3 - k^3 - n = 3dkk + 3ddk + (ddd-n) = 0
    for d in 1..=n.nth_root(3) {
        // axx + bx + c = 0 => x = (-b ± √(b^2 - 4ac)) / 2a
        let a = 3 * d;
        let b = 3 * d * d;
        let c = d * d * d - n;
        let discriminant = b * b - 4 * a * c;
        if discriminant < 0 {
            continue;
        }
        let sqrt_discriminant = discriminant.sqrt();
        let k = (-b + sqrt_discriminant) / (2 * a);
        let y = k;
        let x = d + k;
        if 0 < y && x.pow(3) - y.pow(3) == n {
            println!("{x} {y}");
            return;
        }
    }
    println!("-1");
}
