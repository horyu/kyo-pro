#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        a: usize,
        n: usize,
    };
    // https://atcoder.jp/contests/abc414/editorial/13452
    let ispal = |mut x: usize, b: usize| {
        let mut ss = vec![];
        while 0 < x {
            ss.push(x % b);
            x /= b;
        }
        let m = ss.len() / 2;
        izip!(ss.iter(), ss.iter().rev())
            .take(m)
            .all(|(&x, &y)| x == y)
    };

    let mut vv = vec![];
    let mut pow10 = vec![1];
    let mut len = 1;
    loop {
        while pow10.len() < len {
            pow10.push(pow10.last().unwrap() * 10);
        }
        if n < pow10[len - 1] {
            break;
        }
        let mut dd = vec![0; len.div_ceil(2)];
        dd[0] = 1;
        loop {
            let mut sum = 0;
            for i in 0..len {
                let idx = if i < dd.len() { i } else { (len - 1) - i };
                sum += pow10[i] * dd[idx];
            }
            if sum <= n && ispal(sum, a) {
                vv.push(sum);
            }

            let mut has_next = false;
            for i in (0..dd.len()).rev() {
                if dd[i] == 9 {
                    dd[i] = 0;
                    continue;
                }
                dd[i] += 1;
                has_next = true;
                break;
            }
            if !has_next {
                break;
            }
        }
        len += 1;
    }

    let rs = vv.into_iter().sum::<usize>();
    println!("{rs}");
}
