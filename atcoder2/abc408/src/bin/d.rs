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
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        };
        // https://atcoder.jp/contests/abc408/editorial/13166
        // cc[i] := aa[i](i番目までの0の個数) - bb[i](i番目までの1の個数)
        let mut cc = vec![0; n + 1];
        for (i, c) in s.iter().copied().enumerate() {
            cc[i + 1] = cc[i] + if c == '0' { 1 } else { -1 };
        }
        let sum = s.iter().filter(|&&c| c == '1').count() as i32;
        let mut max = 0;
        let mut res = 0;
        for i in 0..=n {
            res = res.min(cc[i] - max);
            max = max.max(cc[i]);
        }
        let rs = sum + res;
        println!("{rs}");
    }
}

#[allow(dead_code)]
fn main2() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        };
        // vv := 1の個数 → 0の個数 → 1の個数 → ...
        let mut vv = vec![];
        for (c, g) in s.iter().copied().group_by(|&c| c).into_iter() {
            if vv.is_empty() && c == '0' {
                continue;
            }
            vv.push(g.count());
        }
        if s[n - 1] == '0' {
            vv.pop();
        }
        let m = vv.len();
        if m <= 1 {
            println!("0");
            continue;
        }
        // ll[i][j] := vv[..i] において右端が j (0 or 1) の時の最小操作数
        let mut ll = vec![[0, 1 << 60]; m + 1];
        for (i, v) in vv.iter().copied().enumerate() {
            if i % 2 == 0 {
                // 1
                ll[i + 1][0] = ll[i][0] + v;
                ll[i + 1][1] = ll[i][0].min(ll[i][1]);
            } else {
                // 0
                ll[i + 1][0] = ll[i][0];
                ll[i + 1][1] = (ll[i][0] + v).min(ll[i][1] + v);
            }
        }

        // rr[i][j] := vv[i..] において左端が j (0 or 1) の時の最小操作数
        let mut rr = vec![[0; 2]; m + 1];
        rr[m][1] = 1 << 60;
        for (i, v) in vv.iter().copied().enumerate().rev() {
            if i % 2 == 0 {
                // 1
                rr[i][0] = rr[i + 1][0] + v;
                rr[i][1] = rr[i + 1][0].min(rr[i + 1][1]);
            } else {
                // 0
                rr[i][0] = rr[i + 1][0];
                rr[i][1] = (rr[i + 1][0] + v).min(rr[i + 1][1] + v);
            }
        }
        let mut rs = usize::MAX;
        for i in 0..=m {
            // vv[..i] = 0, vv[i..] = 1
            rs = rs.min(ll[i][0] + rr[i][1]);
            // vv[..i] = 1, vv[i..] = 0
            rs = rs.min(ll[i][1] + rr[i][0]);
        }
        // println!("{vv:?}");
        // println!("{ll:?}");
        // println!("{rr:?}");
        println!("{rs}");
    }
}
