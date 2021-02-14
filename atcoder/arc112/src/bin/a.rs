#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        t: usize,
        lrlr: [(isize, isize); t]
    };
    // l <= a,b,c <= r
    // a = b + c
    // b=l,c=l から b+c=r となるように組み合わせ
    // bとcと/dev/nullにr-2lを割り当てる組み合わせの通り
    // r-2l 個の玉とパーティション2個の並び替え
    // (3)H(r-2l) = (r-2l+2)C(2)
    for (l, r) in lrlr {
        if r - 2 * l < 0 {
            println!("0");
        } else {
            println!("{}", (r + 2 - 2 * l) * (r + 1 - 2 * l) / 2);
        }
    }
}
