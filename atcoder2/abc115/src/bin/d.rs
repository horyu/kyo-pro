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
        mut x: usize,
    };
    // B@P@B
    let mut pp = vec![1usize];
    for i in 0..n {
        pp.push(pp[i] * 2 + 1);
    }
    let mut ss = vec![1usize];
    for i in 0..n {
        ss.push(ss[i] * 2 + 3);
    }
    // level 2
    // 1234567890123
    // BBPPPBPBPPPBB
    // 0012334456777
    let mut rs = 0usize;
    for l in (1..=n).rev() {
        // eprint!("{rs}: ");
        match x.cmp(&(ss[l - 1] + 1)) {
            Ordering::Less => {
                // eprintln!("{l}< {x}->{}", x.saturating_sub(1));
                x = x.saturating_sub(1);
            }
            Ordering::Equal => {
                // eprintln!("{l}= {}", pp[l - 1]);
                rs += pp[l - 1];
                break;
            }
            Ordering::Greater => {
                if x == ss[l] {
                    // eprintln!("{l}> !+{}", pp[l]);
                    rs += pp[l];
                    break;
                } else {
                    // eprintln!("{l}> {x}->{}", x - (ss[l - 1] + 2));
                    rs += pp[l - 1] + 1;
                    x -= ss[l - 1] + 2;
                }
            }
        }
    }
    if x == 1 {
        rs += 1;
    }
    println!("{rs}");
}
