#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut odds = 0;
    let mut amari2 = 0;
    let mut space4 = 0;
    for a in aa {
        match a % 4 {
            0 => space4 += 1,
            2 => amari2 += 1,
            _ => odds += 1,
        }
    }
    if amari2 == 0 {
        println!("{}", ["No", "Yes"][(odds <= (space4 + 1)) as usize])
    } else {
        // 2あまりは2個以上あれば打ち消しあえて一端は4の隣
        amari2 = 1;
        println!(
            "{}",
            ["No", "Yes"][((odds + amari2) <= (space4 + 1)) as usize]
        )
    }
}
