#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        aaa: [Chars; h]
    };
    let aaa = aaa
        .into_iter()
        .map(|aa| {
            aa.into_iter()
                .map(|a| if a == '+' { 1isize } else { -1 })
                .collect_vec()
        })
        .collect_vec();
    if (h, w) == (1, 1) {
        println!("Draw");
        return;
    }
    let mut v = vec![vec![0isize; w]; h];
    for y in (0..(h - 1)).rev() {
        v[y][w - 1] = v[y + 1][w - 1] + (-1isize).pow(((1 + y + w) % 2) as u32) * aaa[y + 1][w - 1];
    }
    for x in (0..(w - 1)).rev() {
        v[h - 1][x] = v[h - 1][x + 1] + (-1isize).pow(((1 + h + x) % 2) as u32) * aaa[h - 1][x + 1];
    }
    for y in (0..(h - 1)).rev() {
        for x in (0..(w - 1)).rev() {
            v[y][x] = if (y + x) % 2 == 0 {
                (v[y + 1][x] + aaa[y + 1][x]).max(v[y][x + 1] + aaa[y][x + 1])
            } else {
                (v[y + 1][x] - aaa[y + 1][x]).min(v[y][x + 1] - aaa[y][x + 1])
            };
        }
    }
    println!(
        "{}",
        match v[0][0] {
            0 => "Draw",
            1..=std::isize::MAX => "Takahashi",
            _ => "Aoki",
        }
    );
}
