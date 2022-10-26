#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        mut t: Chars,
    };
    if t < s {
        std::mem::swap(&mut s, &mut t);
    }
    let mut ds = 0;
    let mut dt = 0;
    for (&cs, &ct) in izip!(&s, &t) {
        if cs != ct {
            if cs != '0' {
                ds += 1;
            }
            if ct != '0' {
                dt += 1;
            }
        }
    }
    if (ds + dt).is_odd() {
        println!("-1");
        return;
    }
    let mut rs = vec!['0'; n];
    let mut dd = (ds + dt) / 2;
    // dbg!(dd);
    for (i, (&cs, &ct)) in izip!(&s, &t).enumerate() {
        if cs != ct {
            if ct == '1' && 0 < dd {
                dd -= 1;
            } else {
                rs[i] = ct;
            }
        }
    }
    for (i, (&cs, &ct)) in izip!(&s, &t).enumerate().rev() {
        if cs != ct {
            // 右から
            if ct == '0' && 0 < dd {
                dd -= 1;
                rs[i] = '1';
            }
        }
    }
    let rs = rs.iter().join("");
    println!("{rs}");
}
