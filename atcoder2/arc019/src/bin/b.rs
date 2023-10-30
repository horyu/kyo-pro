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
        aa: Chars,
    };
    let n = aa.len();
    let mut l = 0;
    let mut r = n - 1;
    let mut diff = 0;
    while l <= r && r < n {
        if aa[l] != aa[r] {
            diff += 1;
        }
        l += 1;
        r = r.saturating_sub(1);
    }
    dbg!(l, r, diff);
    let mut rs = 0;
    let mut l = 0;
    let mut r = n - 1;
    while l <= r && r < n {
        if aa[l] == aa[r] {
            if l == r {
                if 1 <= diff {
                    rs += 25;
                }
            } else {
                rs += 25 * 2;
            }
        } else {
            if diff == 1 {
                rs += 24 * 2;
            } else {
                rs += 25 * 2;
            }
        }
        l += 1;
        r = r.saturating_sub(1);
    }
    println!("{rs}");
}
