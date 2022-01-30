#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        hh: [isize; n],
        uuvv: [(Usize1, Usize1); m],
    };
    let mut vvv = vec![vec![]; n];
    for (u, v) in uuvv {
        vvv[u].push(v);
        vvv[v].push(u);
    }

    let mut qq = VecDeque::new();
    let mut pushed = vec![false; n];
    let mut poped = vec![false; n];
    let mut cc = vec![std::isize::MIN >> 2; n];

    qq.push_back(0);
    pushed[0] = true;
    cc[0] = 0;

    while let Some(q) = qq.pop_front() {
        poped[q] = true;
        let q_c = cc[q];
        for &v in &vvv[q] {
            let c = q_c
                + if hh[q] >= hh[v] {
                    hh[q] - hh[v]
                } else {
                    2 * (hh[q] - hh[v])
                };
            if !pushed[v] {
                qq.push_back(v);
                pushed[v] = true;
            }
            if cc[v] < c {
                cc[v] = c;
                // 更新が行われ、qqに入ってないなら再び見る
                if poped[v] {
                    qq.push_back(v);
                    poped[v] = false;
                }
            }
        }
    }

    let rs = cc.iter().max().unwrap();
    println!("{rs}");
}
