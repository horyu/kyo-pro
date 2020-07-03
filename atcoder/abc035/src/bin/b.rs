#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars,
        t: usize
    };
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut hatena: i32 = 0;
    for c in s {
        match c {
            'L' => x -= 1,
            'R' => x += 1,
            'U' => y += 1,
            'D' => y -= 1,
            '?' => hatena += 1,
            _ => unreachable!(),
        }
    }
    let wa = x.abs() + y.abs();
    if t == 1 {
        println!("{}", wa + hatena);
    } else {
        if wa >= hatena {
            println!("{}", wa - hatena);
        } else {
            println!("{}", (hatena - wa) % 2);
        }
    }
}
