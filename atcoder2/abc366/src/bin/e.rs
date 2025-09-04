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
        d: isize,
        xxyy: [(isize, isize); n],
    };
    // https://atcoder.jp/contests/abc366/editorial/10640
    const M: usize = 2e6 as usize;
    const MI: isize = M as isize;
    let ni = n as isize;
    let xx = xxyy.iter().copied().map(|xy| xy.0).collect_vec();
    let yy = xxyy.iter().copied().map(|xy| xy.1).collect_vec();
    let idx = |k: isize| -> usize {
        if k < 0 {
            (2 * MI + 1 + k) as usize
        } else {
            k as usize
        }
    };
    let calc = |mut vv: Vec<isize>| {
        let mut ss = vec![0; 2 * M + 1];
        vv.sort_unstable();
        let mut i = 0;
        ss[idx(-MI)] = vv.iter().sum::<isize>() + ni * MI;
        for x in (-MI + 1)..=MI {
            while i < n && vv[i] < x {
                i += 1;
            }
            ss[idx(x)] = ss[idx(x - 1)] + 2 * (i as isize) - ni;
        }
        ss.sort_unstable();
        ss
    };

    let pp = calc(xx);
    let qq = calc(yy);
    let mut rs = 0;
    let mut j = 0;
    for i in (0..=(2 * M)).rev() {
        while j <= 2 * M && pp[i] + qq[j] <= d {
            j += 1;
        }
        rs += j;
    }
    println!("{rs}");
}
