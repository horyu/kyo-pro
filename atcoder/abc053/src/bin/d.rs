#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut hm = std::collections::HashMap::new();
    for a in aa {
        let e = hm.entry(a).or_insert(0usize);
        if *e == 2 {
            // 同じのが3個になったら1個に圧縮
            *e = 1;
        } else {
            *e += 1;
        }
    }
    let nums = hm.len();
    let surplus = hm.into_iter().map(|(_, v)| v - 1).sum::<usize>();
    if surplus == 0 {
        println!("{}", nums);
    } else {
        // 1
        // 1 2
        // 1 2 3
        let rs = match surplus % 2 {
            0 => nums,
            1 => nums - 1,
            _ => unreachable!(),
        };
        println!("{}", rs)
    }
}
