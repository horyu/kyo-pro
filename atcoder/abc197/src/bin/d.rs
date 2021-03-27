#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        x0: isize,
        y0: isize,
        xn2: isize,
        yn2: isize,
    };
    // https://ja.wikipedia.org/wiki/%E6%AD%A3%E5%A4%9A%E8%A7%92%E5%BD%A2
    // 1辺の長さ a の正n角形の対角線の長さ a / sin(PI/n)
    // a = 対角線の長さ * sin (PI / n)
    use std::f64::consts::PI;
    // poからpn2 へのベクトルを - (90 * (n - 2))/ n 度回転させて長さを正す
    let x = (xn2 - x0) as f64;
    let y = (yn2 - y0) as f64;
    let theta = (-1.0) * (90 * (n - 2)) as f64 * PI / 180.0 / (n as f64);
    let mul = (PI / (n as f64)).sin();
    let xx = mul * (theta.cos() * x - theta.sin() * y);
    let yy = mul * (theta.sin() * x + theta.cos() * y);
    println!("{} {}", x0 as f64 + xx, y0 as f64 + yy);
}
