#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        aa: [usize; n],
    };
    let aa = aa.into_iter().map(|a| a % (x + y)).collect_vec();
    // 勝敗1手前
    // 相手の取れる最後の山を取って取れなくする || 相手は既に詰みで自分だけ取って問題ない山を取る
    let rs = match x.cmp(&y) {
        std::cmp::Ordering::Less => {
            todo!()
        },
        std::cmp::Ordering::Equal => {
            aa.into_iter().fold(0, |acc, a| (acc + a / x) % 2) == 1
        },
        std::cmp::Ordering::Greater => {
            todo!()
        },
    };
    println!("{}", ["First", "Second"][rs as usize]);
}
