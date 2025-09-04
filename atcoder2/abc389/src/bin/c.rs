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
        q: usize,
    };
    let mut ft = ac_library::FenwickTree::new(3e5 as usize, 0usize);
    let mut l = 0;
    let mut r = 0;
    for _ in 0..q {
        input! {t: u32};
        match t {
            1 => {
                input! {x: usize};
                ft.add(r, x);
                r += 1;
            }
            2 => {
                l += 1;
            }
            3 => {
                input! {k: Usize1};
                let rs = ft.sum(l..(l + k));
                println!("{rs}");
            }
            _ => unreachable!(),
        }
        // eprintln!("[{l}-{r}]{t}: {}", (0..r).map(|i| ft.sum(i..=i)).join(" "));
    }
}
