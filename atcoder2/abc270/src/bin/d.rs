#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        mut n: usize,
        k: usize,
        mut aa: [usize; k],
    };
    let mut rs = 0;
    let mut r = k - 1;
    while 0 < n {
        while n < aa[r] {
            r -= 1;
        }
        if 1 < r && aa[r] < 2 * aa[r - 1] && 2 * aa[r - 1] + aa[r] == n {
            rs += 2 * aa[r - 1];
            break;
        }
        rs += aa[r];
        n -= aa[r];
        if n == 0 {
            break;
        }
        while n < aa[r] {
            r -= 1;
        }
        if 1 < r && aa[r] < 2 * aa[r - 1] && 2 * aa[r - 1] + aa[r] == n {
            break;
        }
        n -= aa[r];
    }
    println!("{rs}");
}
