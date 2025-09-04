#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use ac_library::ModInt1000000007;
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
        a: usize,
        b: usize,
    };
    let mut rs = ModInt1000000007::new(2);
    rs = rs.pow(n as u64) - 1;
    // dbg!(&rs);
    for x in [a, b] {
        let mut tmp = ModInt1000000007::new(1);
        // dbg!((n - x + 1)..=n, 1..=x);
        for i in (n - x + 1)..=n {
            tmp *= i;
        }
        for j in 1..=x {
            tmp /= j;
        }
        rs -= tmp;
        // dbg!(&tmp);
    }
    println!("{rs}");
}
