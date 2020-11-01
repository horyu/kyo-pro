#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut h: usize,
        mut w: usize,
        mut aaa: [Chars; h]
    };
    'outer: loop {
        for y in 0..h {
            let aa = &aaa[y];
            if aa.iter().all(|&c| c == '.') {
                aaa.remove(y);
                h -= 1;
                continue 'outer;
            }
        }
        for x in 0..w {
            if (0..h).all(|y| aaa[y][x] == '.') {
                for y in 0..h {
                    aaa[y].remove(x);
                }
                w -= 1;
                continue 'outer;
            }
        }
        break;
    }
    for aa in aaa {
        println!("{}", aa.iter().collect::<String>());
    }
}
