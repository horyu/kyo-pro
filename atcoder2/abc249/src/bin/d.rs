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
        aa: [usize; n],
    };
    let hm = aa.into_iter().counts();
    let mut rs = 0usize;
    for &a in hm.keys() {
        for x in 1..=a.sqrt() {
            let y = a / x;
            if x * y != a {
                continue;
            }
            rs += if x == y {
                if y == a {
                    // 1*1=1
                    let c = *hm.get(&a).unwrap_or(&0);
                    c.pow(3)
                } else {
                    // x*x = a
                    let c = *hm.get(&x).unwrap_or(&0);
                    c.pow(2) * hm.get(&a).unwrap_or(&0)
                }
            } else {
                if y == a {
                    // 1*a=a; a*1=a;
                    let c = *hm.get(&a).unwrap_or(&0);
                    2 * c.pow(2) * hm.get(&x).unwrap_or(&0)
                } else {
                    // xy=a; yx=a
                    2 * hm.get(&x).unwrap_or(&0)
                        * hm.get(&y).unwrap_or(&0)
                        * hm.get(&a).unwrap_or(&0)
                }
            };
        }
    }
    println!("{rs}");
}
