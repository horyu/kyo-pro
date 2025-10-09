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
        rt: i128,
        ct: i128,
        ra: i128,
        ca: i128,
        n: usize,
        m: usize,
        l: usize,
        ssaa: [(char, i128); m],
        ttbb: [(char, i128); l],
    };
    // 青木君の位置を原点として、高橋君の移動のみで考える
    let mut rt = rt - ra;
    let mut ct = ct - ca;
    // 偶奇が不一致の場合、永遠に交差しない
    if (rt.abs() + ct.abs()) % 2 != 0 {
        println!("0");
        return;
    }
    let mut ssaa = VecDeque::from(ssaa);
    let mut ttbb = VecDeque::from_iter(ttbb.into_iter().map(|(c, b)| {
        (
            match c {
                'U' => 'D',
                'D' => 'U',
                'L' => 'R',
                'R' => 'L',
                _ => unreachable!(),
            },
            b,
        )
    }));
    let mut rs = 0;
    while let (Some(&(s, a)), Some(&(t, b))) = (ssaa.front(), ttbb.front()) {
        let mut dxdy = (0, 0);
        for c in [s, t] {
            match c {
                'U' => dxdy.0 -= 1,
                'D' => dxdy.0 += 1,
                'L' => dxdy.1 -= 1,
                'R' => dxdy.1 += 1,
                _ => unreachable!(),
            }
        }
        let min = a.min(b);
        if dxdy == (0, 0) {
            if (rt, ct) == (0, 0) {
                rs += min;
            }
        } else {
            let nrt = rt + dxdy.0 * min;
            let nct = ct + dxdy.1 * min;
            // 原点と交差したか
            if rt == nrt {
                if rt == 0 && ct != 0 && ct * nct <= 0 {
                    rs += 1;
                }
            } else if ct == nct {
                if ct == 0 && rt != 0 && rt * nrt <= 0 {
                    rs += 1;
                }
            } else {
                // 原点を通る45度の直線で原点を通る
                if rt.abs() == ct.abs()
                    && nrt.abs() == nct.abs()
                    && rt * nrt <= 0
                    && ct * nct <= 0
                    && (rt, ct) != (0, 0)
                {
                    rs += 1;
                }
            }
            rt = nrt;
            ct = nct;
        }

        match a.cmp(&b) {
            Ordering::Less => {
                ssaa.pop_front();
                ttbb[0].1 -= min;
            }
            Ordering::Equal => {
                ssaa.pop_front();
                ttbb.pop_front();
            }
            Ordering::Greater => {
                ssaa[0].1 -= min;
                ttbb.pop_front();
            }
        }
    }
    println!("{rs}");
}
