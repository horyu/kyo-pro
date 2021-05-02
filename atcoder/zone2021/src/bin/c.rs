#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aabbccddee: [[usize; 5]; n]
    };
    let vvv = (0usize..5)
        .map(|i| {
            let mut vv = aabbccddee
                .iter()
                .enumerate()
                .map(|(index, abcde)| (index, abcde[i]))
                .collect_vec();
            vv.sort_unstable_by_key(|&(_index, num)| num);
            vv.reverse();
            vv
        })
        .collect_vec();
    // vvv をそれぞれ上から以下のどちらかを満たすまで見ていく
    // - 3つ以上のパラメータで特定個人が出る（残り２つのパラメータはそれぞれの一番高い人）
    // - 4つ以上のパラメータで特定の二人が出る（残り１つのパラメータはそれの一番高い人）
    // println!("{}", );
}
