#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        w: usize,
        sss: [Chars; h]
    };
    let mut rs = 0;
    // 0  1  1  0  1  0  -  1
    // .. .. .. .. .# .# .# .#
    // .. .# #. ## .. .# #. ##
    // 1  -  0  1  0  1  1  0
    // #. #. #. #. ## ## ## ##
    // .. .# #. ## .. .# #. ##
    // => #が奇数 +1
    let mut arr = ['_'; 4];
    for y in 0..(h - 1) {
        for x in 0..(w - 1) {
            arr[0] = sss[y][x];
            arr[1] = sss[y][x + 1];
            arr[2] = sss[y + 1][x];
            arr[3] = sss[y + 1][x + 1];
            rs += (arr.iter().filter(|&&c| c == '#').count() % 2 == 1) as i32;
        }
    }
    println!("{}", rs);
}
