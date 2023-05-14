#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        h: usize,
        w: usize,
        t: usize,
        aaa: [Chars; h],
    };
    let (mut sx, mut sy) = (0, 0);
    let (mut gx, mut gy) = (0, 0);
    for (i, aa) in aaa.iter().enumerate() {
        for (j, &a) in aa.iter().enumerate() {
            match a {
                'S' => {
                    (sx, sy) = (i, j);
                }
                'G' => {
                    (gx, gy) = (i, j);
                }
                _ => (),
            }
        }
    }
    let udlr = |i: usize, j: usize| -> Vec<(usize, usize)> {
        let mut vv = vec![];
        if 0 < i {
            vv.push((i - 1, j));
        }
        if i < h - 1 {
            vv.push((i + 1, j));
        }
        if 0 < j {
            vv.push((i, j - 1));
        }
        if j < w - 1 {
            vv.push((i, j + 1));
        }
        vv.retain(|&(vi, vj)| aaa[vi][vj] != '#');
        vv
    };
    let mut qq = VecDeque::new();
    qq.push_back((sx, sy, 0));
    let mut dd = vec![vec![!0usize; w]; h];
    dd[sx][sy] = 0;
    let mut oo = vec![];
    let mut s2o = vec![];
    while let Some((qi, qj, qd)) = qq.pop_front() {
        if t <= qd {
            continue;
        }
        if aaa[qi][qj] == 'o' {
            oo.push((qi, qj));
            s2o.push(qd);
        }
        for (i, j) in udlr(qi, qj) {
            if dd[i][j] == !0 {
                dd[i][j] = qd + 1;
                qq.push_back((i, j, qd + 1));
            }
        }
    }
    if dd[gx][gy] == !0 {
        println!("-1");
        return;
    }

    let olen = oo.len();

    let mut qq = VecDeque::new();
    qq.push_back((gx, gy, 0));
    let mut dd = vec![vec![!0usize; w]; h];
    dd[gx][gy] = 0;
    let mut g2o = vec![1usize << 60; olen];
    while let Some((qi, qj, qd)) = qq.pop_front() {
        if t <= qd {
            continue;
        }
        if aaa[qi][qj] == 'o' {
            if let Some(oi) = oo.iter().position(|&o| o == (qi, qj)) {
                g2o[oi] = qd;
            }
        }
        for (i, j) in udlr(qi, qj) {
            if dd[i][j] == !0 {
                dd[i][j] = qd + 1;
                qq.push_back((i, j, qd + 1));
            }
        }
    }

    let mut o2o = vec![vec![1usize << 60; olen]; olen];
    for (oi, (ox, oy)) in oo.iter().copied().enumerate() {
        let mut qq = VecDeque::new();
        qq.push_back((ox, oy, 0));
        let mut dd = vec![vec![!0usize; w]; h];
        while let Some((qi, qj, qd)) = qq.pop_front() {
            if t <= qd {
                continue;
            }
            if aaa[qi][qj] == 'o' {
                if let Some(oj) = oo.iter().position(|&o| o == (qi, qj)) {
                    o2o[oi][oj] = o2o[oi][oj].min(qd);
                }
            }
            for (i, j) in udlr(qi, qj) {
                if dd[i][j] == !0 {
                    dd[i][j] = qd + 1;
                    qq.push_back((i, j, qd + 1));
                }
            }
        }
    }
    // eprintln!("s2o:{}", s2o.iter().join(" "));
    // eprintln!("g2o:{}", g2o.iter().join(" "));
    // for o2o in &o2o {
    //     eprintln!("!{}", o2o.iter().join(" "));
    // }

    let mut dp = vec![vec![1usize << 60; olen]; 1 << olen];
    #[allow(clippy::manual_memcpy)]
    for i in 0..olen {
        dp[1 << i][i] = s2o[i];
    }
    for s in 1..(1 << olen) {
        for v in 0..olen {
            for u in 0..olen {
                if 0 == (s & (1 << u)) {
                    continue;
                }
                if 0 == (s & (1 << v)) {
                    // if w < 4 {
                    //     eprintln!("{s:b} {u} {v}: {} -> {}", dp[s | (1 << v)][v], dp[s][u] + o2o[u][v]);
                    // }
                    dp[s | (1 << v)][v] = dp[s | (1 << v)][v].min(dp[s][u] + o2o[u][v]);
                }
            }
        }
    }
    let mut rs = 0;
    for (s, vv) in dp.iter().enumerate() {
        for i in 0..olen {
            if 0 == (s & (1 << i)) {
                continue;
            }
            for j in 0..olen {
                if 0 == (s & (1 << j)) {
                    continue;
                }
                let sum = vv[j] + g2o[j];
                if sum <= t {
                    rs = rs.max(s.count_ones());
                }
                // eprintln!("{s:b} {i} {sum}({} {} {})", s2o[i], vv[j], g2o[j]);
            }
        }
    }
    println!("{rs}");
}
