#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        // x: usize,
        // y: usize,
        // a: usize,
        // b: usize,
        // c: usize,
        xy: [usize; 2],
        abc: [usize; 3],
    };
    if xy.iter().product::<usize>() < abc.iter().sum::<usize>() {
        println!("No");
        return;
    }
    let [x, y] = xy[..] else {return;};
    for abc in abc.into_iter().permutations(3) {
        let [a,b,c] = abc[..] else {return;};
        for (x0, y0) in [(x, y), (y, x)] {
            let x1 = x0.saturating_sub(a.div_ceil(y0));
            if x1 == 0 {
                continue;
            }
            for (x1, y1) in [(x1, y0), (y0, x1)] {
                let x2 = x1.saturating_sub(b.div_ceil(y1));
                if c <= x2 * y1 {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
