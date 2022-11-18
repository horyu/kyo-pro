#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library_rs::modint::ModInt1000000007 as Mint;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        ss: [Chars; 2],
    };
    let mut rs = Mint::new(1);
    let mut i;
    let mut pre_tate;
    if ss[0][0] == ss[1][0] {
        i = 1;
        pre_tate = true;
        rs *= 3;
    } else {
        i = 2;
        pre_tate = false;
        rs *= 6;
    }
    while i < n {
        if ss[0][i] == ss[1][i] {
            rs *= [1, 2][pre_tate as usize];
            i += 1;
            pre_tate = true;
        } else {
            rs *= [3, 2][pre_tate as usize];
            i += 2;
            pre_tate = false;
        }
    }
    println!("{rs}");
}
