#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut x: Chars,
        m: i128
    };
    x.reverse(); // p進数にするときに enum で pow とあわせやすくするため逆順に
    let x = x
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as i128)
        .collect_vec();
    if x.len() == 1 {
        println!("{}", (x[0] <= m) as usize);
        return;
    }
    let d = x.iter().max().unwrap();
    let mut left = d + 1;
    // 二部探索初期値チェック
    if p_value(&x, left) > m {
        println!("0");
        return;
    }
    let mut right = std::i128::MAX - left;
    // https://twitter.com/meguru_comp/status/697008509376835584
    while (right - left) > 1 {
        let mid = (left + right) / 2;
        if p_value(&x, mid) <= m {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left - d);
}

fn p_value(x: &[i128], p: i128) -> i128 {
    let mut rs = 0i128;
    for (i, &num) in x.iter().enumerate() {
        if num == 0 {
            continue;
        }
        if let Some(pow) = p.checked_pow(i as u32) {
            if let Some(tmp) = pow.checked_mul(num) {
                if let Some(next_rs) = rs.checked_add(tmp) {
                    rs = next_rs;
                    continue;
                }
            }
        }
        return std::i128::MAX;
    }
    rs
}
