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
        n: usize,
    };
    // https://namonakimichi.hatenablog.com/entry/2015/08/11/143312
    let depth = n.ilog2() + 1;

    let mut tmp = 1;
    let mut turn = 0;
    if depth.is_even() {
        while tmp <= n {
            if turn.is_even() {
                tmp *= 2;
            } else {
                tmp = tmp * 2 + 1;
            }
            turn += 1;
        }
    } else {
        while tmp <= n {
            if turn.is_even() {
                tmp = tmp * 2 + 1;
            } else {
                tmp *= 2;
            }
            turn += 1;
        }
    }

    let rs = if turn.is_even() { "Takahashi" } else { "Aoki" };
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
    };
    // https://qiita.com/hamamu/items/55433210be3c47a4dd72
    let mut y = n + 1;
    loop {
        let z = y.div_ceil(2);
        if z == 1 {
            println!("Aoki");
            return;
        }
        y = (z - 1).div_ceil(2);
        if y == 1 {
            println!("Takahashi");
            return;
        }
    }
}
