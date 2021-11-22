#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        sx: isize,
        sy: isize,
        tx: isize,
        ty: isize,
    };
    let mut x = sx;
    let mut y = sy;
    let mut rs = String::new();
    // 上*右*
    while y != ty {
        y += 1;
        rs.push('U');
    }
    while x != tx {
        x += 1;
        rs.push('R');
    }
    // 下*左*
    while y != sy {
        y -= 1;
        rs.push('D');
    }
    while x != sx {
        x -= 1;
        rs.push('L');
    }
    // 左上*右*下
    x -= 1;
    rs.push('L');
    while y != ty + 1 {
        y += 1;
        rs.push('U');
    }
    while x != tx {
        x += 1;
        rs.push('R');
    }
    y -= 1;
    rs.push('D');
    // 右下*左*上
    x += 1;
    rs.push('R');
    while y != sy - 1 {
        y -= 1;
        rs.push('D');
    }
    while x != sx {
        x -= 1;
        rs.push('L');
    }
    // y -= 1;
    rs.push('U');

    println!("{}", rs);
}
