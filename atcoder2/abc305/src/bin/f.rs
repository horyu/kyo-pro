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
    let mut stdin =
        proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {
        n: usize,
        m: usize,
    };
    input! {
        size: usize,
        vv: [usize; size],
    };

    let mut crr = 1;
    let mut g = vec![vec![]; n + 1];
    g[crr] = vv;
    let mut ff = vec![!0; n + 1];

    while !g[crr].contains(&n) {
        let mut next = ff[crr];
        for &i in &g[crr] {
            if ff[i] == !0 {
                next = i;
                break;
            }
        }
        println!("{next}");

        if ff[next] == !0 {
            ff[next] = crr;
        }
        crr = next;

        input! {
            size: usize,
            vv: [usize; size],
        };
        g[crr] = vv;
    }

    println!("{n}");
}
