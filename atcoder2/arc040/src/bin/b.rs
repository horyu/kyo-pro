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
        n: usize,
        r: usize,
        s: Chars,
    };
    let mut ttff = s.iter().copied().map(|c| c == 'o').collect_vec();
    let Some(last) = ttff.iter().rposition(|&b| !b) else {
        println!("0");
        return;
    };
    if last < r {
        println!("1");
        return;
    }
    let mut pos = 0;
    let mut rs = 1;
    while pos <= last.saturating_sub(r) {
        if ttff[pos] {
            pos += 1;
            rs += 1;
        } else {
            for i in pos..(pos + r) {
                ttff[i] = true;
            }
            pos += 1;
            rs += 2;
        }
    }
    // dbg!(pos, rs);
    // eprintln!(
    //     "{}",
    //     ttff.iter().map(|&b| if b { 'o' } else { 'x' }).join("")
    // );

    println!("{rs}");
}
