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
        mut n: Usize1,
    };
    // https://atcoder.jp/contests/abc363/editorial/10464
    if n == 0 {
        println!("0");
        return;
    }

    for d in 1.. {
        let x = (d + 1) / 2;
        if n <= 9 * ten(x - 1) {
            let mut rs = (ten(x - 1) + n - 1).to_string().chars().collect_vec();
            rs.resize(d, ' ');
            for i in x..d {
                rs[i] = rs[d - 1 - i];
            }
            let rs = rs.into_iter().join("");
            println!("{rs}");
            return;
        } else {
            n -= 9 * ten(x - 1);
        }
    }
}

fn ten(a: usize) -> usize {
    if a == 0 {
        1
    } else {
        10 * ten(a - 1)
    }
}
