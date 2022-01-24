#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
    };
    let mut tt = vec![];
    let mut aaa = vec![];
    for _ in 0..n {
        input! {
            t: usize,
            k: usize,
            aa: [Usize1; k],
        };
        tt.push(t);
        aaa.push(aa);
    }
    let mut qq = VecDeque::new();
    let mut vv = vec![0usize; n];
    qq.push_back(n - 1);
    vv[n - 1] = tt[n - 1];
    while let Some(q) = qq.pop_front() {
        for &a in &aaa[q] {
            if vv[a] == 0 {
                vv[a] = tt[a];
                qq.push_back(a);
            }
        }
    }
    let rs = vv.into_iter().sum::<usize>();
    println!("{rs}");
}
