#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    };
    let xx = s.into_iter().map(|c| usize::from(c == 'b')).collect_vec();
    let mut rs = 0usize;
    let mut cc = [0; 2];
    let mut l = 0;
    // 尺取法
    for r in 0..n {
        cc[xx[r]] += 1;
        while 0 < l && cc[1] <= b {
            l -= 1;
            cc[xx[l]] += 1;
        }
        while l < r && b <= cc[1] {
            cc[xx[l]] -= 1;
            l += 1;
        }
        while a <= cc[0] && cc[1] < b {
            rs += 1;
            eprintln!("!{l}-{r} {cc:?} {rs}");
            if r <= l {
                break;
            }
            cc[xx[l]] -= 1;
            l += 1;
        }
        eprintln!("{l} {r} {cc:?} {rs}");
    }
    println!("{rs}");
}
