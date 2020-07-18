#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        w: usize,
        h: usize,
        n: usize,
        xxyyss: [(usize, usize, usize); n]
    };
    // (x+0.5, y+0.5) の部分が塗られるかどうかを数える
    let mut area = vec![vec![true; w]; h];
    for (x, y, s) in xxyyss {
        match s {
            1 => {
                for y in 0..h {
                    for xi in 0..x {
                        area[y][xi] = false;
                    }
                }
            }
            2 => {
                for y in 0..h {
                    for xi in x..w {
                        area[y][xi] = false;
                    }
                }
            }
            3 => {
                for x in 0..w {
                    for yi in 0..y {
                        area[yi][x] = false;
                    }
                }
            }
            4 => {
                for x in 0..w {
                    for yi in y..h {
                        area[yi][x] = false;
                    }
                }
            }
            _ => unreachable!(),
        };
    }
    let count = area.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |accc, &point| accc + (point as i32))
    });
    println!("{}", count);
}
