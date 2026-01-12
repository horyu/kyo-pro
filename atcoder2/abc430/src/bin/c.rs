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
    let s = s.into_iter().collect_vec();

    let mut rs = 0usize;
    let mut ra = 0usize; // [l, ra) で a の個数が A 以上になる最小の ra
    let mut rb = 0usize; // [l, rb) で b の個数が B 未満を満たす最大の rb
    let mut cnt_a = 0usize;
    let mut cnt_b = 0usize;

    for l in 0..n {
        if 0 < l {
            let prev = s[l - 1];
            if l - 1 < ra && prev == 'a' {
                cnt_a -= 1;
            }
            if l - 1 < rb && prev == 'b' {
                cnt_b -= 1;
            }
        }
        if ra < l {
            ra = l;
            cnt_a = 0;
        }
        while ra < n && cnt_a < a {
            if s[ra] == 'a' {
                cnt_a += 1;
            }
            ra += 1;
        }

        if rb < l {
            rb = l;
            cnt_b = 0;
        }
        while rb < n {
            if s[rb] == 'b' {
                if b <= cnt_b + 1 {
                    break;
                }
                cnt_b += 1;
            }
            rb += 1;
        }

        if a <= cnt_a && ra <= rb {
            rs += rb - ra + 1;
        }
    }

    println!("{rs}");
}
