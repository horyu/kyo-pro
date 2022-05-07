#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
#![feature(int_log)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        t: usize,
        llrr: [(usize, usize); t]
    };
    for (l, r) in llrr {
        let rlog10 = r.log10();
        // 同じ桁なら全範囲
        if l.log10() == rlog10 {
            println!("{}", r - l + 1);
            continue;
        }
        // r=ABCD としたとき tail(BCD) と init(ABC) を計算できるようにバラす
        let vv = {
            let mut vv = VecDeque::new();
            let mut rr = r;
            while rr > 0 {
                vv.push_front(rr % 10);
                rr /= 10;
            }
            vv
        };
        // 1000~ABCD までを集計
        let m = 10usize.pow(rlog10);
        let mut rs = r - m + 1;
        // A>=2 のとき 3桁以下の数は 1XYZ に含まれるので集計しない
        if vv[0] >= 2 {
            println!("{rs}");
            continue;
        }
        // l..=999, (ABC + 1)..=999, (BCD + 1)..=999 の適切な範囲を集計
        let ll = l
            .max(vv.iter().skip(1).fold(0, |acc, v| acc * 10 + v) + 1)
            .max(vv.iter().take(vv.len() - 1).fold(0, |acc, v| acc * 10 + v) + 1);
        rs += m - ll;
        println!("{rs}");
    }
}
