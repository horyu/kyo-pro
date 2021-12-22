#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut aaa: [[usize; w]; h]
    };
    let mut rs = vec![];
    // 行ごとに偶数枚になるように右に寄せる
    for (i, aa) in aaa.iter_mut().enumerate() {
        for j in 0..(w - 1) {
            if aa[j] % 2 == 1 {
                // aa[j] -= 1;
                aa[j + 1] += 1;
                rs.push(vec![i, j, i, j + 1]);
            }
        }
    }
    // 一番右の列が偶数枚になるように下に寄せる
    for i in 0..(h - 1) {
        let j = w - 1;
        if aaa[i][j] % 2 == 1 {
            // aaa[i][j] -= 1;
            aaa[i + 1][j] += 1;
            rs.push(vec![i, j, i + 1, j]);
        }
    }
    if rs.is_empty() {
        println!("0");
    } else {
        println!(
            "{}\n{}",
            rs.len(),
            rs.into_iter()
                .map(|v| v.into_iter().map(|x| x + 1).join(" "))
                .join("\n")
        );
    }
}
