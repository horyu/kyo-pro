#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        xx: [Usize1; n],
        cc: [usize; n],
    };
    let mut ate = vec![false; n];
    let mut vvv = vec![HashSet::new(); n];
    for (i, &x) in xx.iter().enumerate() {
        vvv[x].insert(i);
    }
    let mut qq: VecDeque<_> = VecDeque::from_iter(0usize..n);
    while let Some(i) = qq.pop_front() {
        if !ate[i] && vvv[i].is_empty() {
            ate[i] = true; // 先に食う
            let x = xx[i];
            vvv[x].remove(&i); // お好きにどうぞ
            qq.push_back(x);
        }
    }
    let mut rs = 0usize;
    for i in 0..n {
        if ate[i] {
           continue; 
        }
        // ループを探す
        let mut hs = HashSet::new();
        let mut next = i;
        while hs.insert(next) {
            ate[next] = true;
            next = xx[next];
        }
        rs += hs.into_iter().map(|i| cc[i]).min().unwrap();
    }
    println!("{rs}")
}
