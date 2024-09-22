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
        hh: [usize; n],
    };
    // 現在の(敷居の高さ, 敷居が有効な幅)の配列
    let mut xxyy = vec![(usize::MAX, 0)];
    let mut vv = vec![];
    let mut turn = 1;
    for h in hh {
        xxyy.push((h, 1));
        turn += h;
        for pos in (0..(xxyy.len() - 1)).rev() {
            let (x, y) = xxyy[pos];
            match x.cmp(&h) {
                Ordering::Less | Ordering::Equal => {
                    turn += (h - x) * y;
                    xxyy.swap_remove(pos);
                    xxyy[pos].1 += y;
                }
                Ordering::Greater => {
                    break;
                }
            }
        }
        vv.push(turn);
    }
    let rs = vv.iter().join(" ");
    println!("{rs}");
}
