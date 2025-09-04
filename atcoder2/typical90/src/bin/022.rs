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
        mut a: u128,
        mut b: u128,
        mut c: u128,
    };
    loop {
        let gcd = a.gcd(&b).gcd(&c);
        if gcd == 1 {
            break;
        }
        a /= gcd;
        b /= gcd;
        c /= gcd;
    }
    let rs = a + b + c - 3;
    println!("{rs}");
}
