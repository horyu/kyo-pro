#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        b: isize,
        c: isize,
    };
    let ch = c / 2;
    let cm = c - 1;
    let cmh = (c - 1) / 2;
    if b == 0 {
        // 初期値 + ただ引くだけ + 引いてから反転
        println!("{}", 1 + ch + if c % 2 == 0 { ch - 1 } else { ch });
        return;
    }
    let mut rs = 0;
    if b > 0 {
        if b - ch > 0 {
            rs += 2;
            // (b - ch)~(b-1)までの1回以上引くだけ + 反転
            if ch > 0 {
                rs += ch + if c % 2 == 0 { ch - 1 } else { ch };
            }
            // 初手反転から (-b-(c-1)/2)~(-b-1) までの1回以上引くだけ + 反転
            if cmh > 0 {
                rs += cmh + if cm % 2 == 0 { cmh - 1 } else { cmh };
            }
        } else {
            // b - ch <= 0
            // -b ~ b
            rs += 2 * b + 1;
            // 初手反転から (-b-(c-1)/2)~(-b-1) までの1回以上引くだけ + 反転
            if cmh > 0 {
                rs += cmh + if cm % 2 == 0 { cmh - 1 } else { cmh };
            }
        }
    } else {
        // b < 0
        if -b - cmh > 0 {
            rs += 2;
            // (b - ch)~(b-1)までの1回以上引くだけ + 反転
            if ch > 0 {
                rs += ch + if c % 2 == 0 { ch - 1 } else { ch };
            }
            // 初手反転から (-b-(c-1)/2)~(-b-1) までの1回以上引くだけ + 反転
            if cmh > 0 {
                rs += cmh + if cm % 2 == 0 { cmh - 1 } else { cmh };
            }
        } else {
            // -b - cmh <= 0
            // -b ~ b
            rs += 2 * b.abs() + 1;
            // (-b-c/2)~(b-1) までの1回以上引くだけ + 反転
            if ch > 0 {
                rs += ch + if c % 2 == 0 { ch - 1 } else { ch };
            }
        }
    }
    println!("{}", rs);
}
