#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

// https://qiita.com/kkdd/items/b3c5e06798e59fe2768e
fn main() {
    input! {
        px: f64,
        py: f64,
        n: usize,
        xxyy: [(f64, f64); n]
    };
    let first = xxyy[0];
    let rs = xxyy
        .into_iter()
        .chain(std::iter::once(first))
        .tuple_windows()
        .fold(1e9, |min: f64, ((ax, ay), (bx, by))| {
            let d = ((px - ax) * (ay - by) - (py - ay) * (ax - bx)).abs()
                / ((ax - bx).powi(2) + (ay - by).powi(2)).sqrt();
            min.min(d)
        });
    println!("{rs}");
}
