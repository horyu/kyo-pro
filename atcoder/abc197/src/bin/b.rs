#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        x: Usize1,
        y: Usize1,
        ss: [Chars; h],
    };
    let mut rs = 1;
    if x > 0 {
        for i in (0..=(x - 1)).rev() {
            if ss[i][y] == '.' {
                rs += 1;
            } else {
                break;
            }
        }
    }
    for i in (x + 1)..h {
        if ss[i][y] == '.' {
            rs += 1;
        } else {
            break;
        }
    }
    if y > 0 {
        for j in (0..=(y - 1)).rev() {
            if ss[x][j] == '.' {
                rs += 1;
            } else {
                break;
            }
        }
    }
    for j in (y + 1)..w {
        if ss[x][j] == '.' {
            rs += 1;
        } else {
            break;
        }
    }

    println!("{}", rs);
}
