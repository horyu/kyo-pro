#![allow(unused_imports)]
use std::f32::consts::FRAC_PI_4;

// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize
    };
    if x == a * a * b {
        println!("0");
        return;
    }
    let x = x as f64;
    let a = a as f64;
    let b = b as f64;
    use std::f64::consts::{FRAC_PI_2, PI};
    let rs = if x > a * a * b / 2.0 {
        // rs < 45度, 半分以上入っている
        // 水は台形型なので空白部分が差分と一致するように計算
        // 1/2 * a * a * tan(rs) * a = a * a * b - x
        // tan(rs) = 2 / a * (b - x/aa)
        (2.0 / a* (b - x / (a * a))).atan()
    } else {
        // rs >= 45度, 半分未満　三角形
        // 1/2 * b * b * tan(2/PI - rs) * a = x
        // tan(2/PI - rs) = 2 * x / abb
        FRAC_PI_2 - (2.0 * x / (a * b * b)).atan()
    } * 180.0
        / PI;
    println!("{}", rs);
}
