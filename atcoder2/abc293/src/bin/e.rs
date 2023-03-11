#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library_rs::ModInt;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_bigint::BigInt;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        a: u64,
        x: u64,
        m: u32,
    };
    if a == 1 {
        println!("{}", x % (m as u64));
        return;
    }
    ModInt::set_modulus(m);
    let rs = f(a, x);
    println!("{rs}");
}

fn f(a: u64, x: u64) -> ModInt {
    if x.is_odd() {
        ModInt::new(a).pow(x - 1) + f(a, x - 1)
    } else {
        if x == 0 {
            ModInt::new(0)
        } else {
            (ModInt::new(a).pow(x / 2) + 1) * f(a, x / 2)
        }
    }
}
