#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        deg: i64,
        dis: f64
    };
    let w = match (dis / 60.0 * 10.0).round() / 10.0 {
        x if x <= 0.2 => 0,
        x if x <= 1.5 => 1,
        x if x <= 3.3 => 2,
        x if x <= 5.4 => 3,
        x if x <= 7.9 => 4,
        x if x <= 10.7 => 5,
        x if x <= 13.8 => 6,
        x if x <= 17.1 => 7,
        x if x <= 20.7 => 8,
        x if x <= 24.4 => 9,
        x if x <= 28.4 => 10,
        x if x <= 32.6 => 11,
        _ => 12,
    };
    let dir = match deg {
        _ if w == 0 => "C",
        y if y < 113 => "N",
        y if y < 338 => "NNE",
        y if y < 563 => "NE",
        y if y < 788 => "ENE",
        y if y < 1013 => "E",
        y if y < 1238 => "ESE",
        y if y < 1463 => "SE",
        y if y < 1688 => "SSE",
        y if y < 1913 => "S",
        y if y < 2138 => "SSW",
        y if y < 2363 => "SW",
        y if y < 2588 => "WSW",
        y if y < 2813 => "W",
        y if y < 3038 => "WNW",
        y if y < 3263 => "NW",
        y if y < 3488 => "NNW",
        _ => "N",
    };
    println!("{} {}", dir, w);
}
