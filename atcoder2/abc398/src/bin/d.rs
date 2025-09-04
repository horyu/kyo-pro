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
        mut r: isize,
        mut c: isize,
        s: Chars,
    };
    let mut hs = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    hs.insert((x, y));
    let mut rs = String::new();
    for dir in s {
        match dir {
            'N' => {
                r += 1;
                x += 1;
            }
            'W' => {
                c += 1;
                y += 1;
            }
            'S' => {
                r -= 1;
                x -= 1;
            }
            'E' => {
                c -= 1;
                y -= 1;
            }
            _ => unreachable!(),
        }
        hs.insert((x, y));
        if hs.contains(&(r, c)) {
            rs.push('1');
        } else {
            rs.push('0');
        }
        // eprintln!("{dir} ({x} {y}) ({r} {c})");
    }
    println!("{rs}");
}
