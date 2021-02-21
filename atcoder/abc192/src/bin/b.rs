#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    for (i, c) in s.iter().enumerate() {
        let j = i + 1;
        // 奇数かつ小文字 or 偶数かつ大文字 => 読みにくい
        if j % 2 == 1 {
            if c.is_ascii_lowercase() {
                continue;
            }
        } else {
            if c.is_ascii_uppercase() {
                continue;
            }
        }
        println!("No");
        return;
    }
    println!("Yes");
}
