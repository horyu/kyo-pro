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
        m: usize,
        q: usize,
        aabbccdd: [(usize, usize, usize, usize); q],
    };
    let nn = ModInt998244353::new(n);
    let mm = ModInt998244353::new(m);
    // (a, c) != 0, aと同じ偶奇行の合計を出す
    let f = |a: usize, b: usize, c: usize, d: usize| -> ModInt998244353 {
        // a行の合計: 初項x=(a - 1)*m+c 項数k=(d - c)/2 + 1, 等差2
        let x = mm * (a - 1) + c;
        let k = ModInt998244353::new((d - c) / 2 + 1);
        let a_row = k * (x * 2 + (k - 1) * 2) / 2;
        // a + 2i行の合計 = a_row + 2*m*k
        // aと同じ偶奇行の合計: 初項a_row 項数l=(b - a)/2 + 1, 等差 2*m*k
        let l = ModInt998244353::new((b - a) / 2 + 1);
        l * (a_row * 2 + (l - 1) * (k * m * 2)) / 2
    };
    for (a, b, c, d) in aabbccdd {
        let mut rs = ModInt998244353::new(0);
        if (a + c).is_even() {
            rs += f(a, b, c, d);

            if a < b && c < d {
                rs += f(a + 1, b, c + 1, d);
            }
        } else {
            if c < d {
                rs += f(a, b, c + 1, d);
            }

            if a < b {
                rs += f(a + 1, b, c, d);
            }
        }
        println!("{rs}");
    }
}
