#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        r: usize,
        _c: usize,
        s: (Usize1, Usize1),
        g: (Usize1, Usize1),
        ccc: [Chars; r],
    };
    // # : std::isize::MIN
    // . : -1
    // s : 0
    // g : std::isize::MAX
    let mut ccc = ccc
        .iter()
        .map(|cc| {
            cc.iter()
                .map(|&c| if c == '#' { std::isize::MIN } else { -1isize })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    ccc[s.0][s.1] = 0;
    ccc[g.0][g.1] = std::isize::MAX;
    let mut queue = std::collections::VecDeque::new();
    let dxdys = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    queue.push_front(s);
    while let Some((x, y)) = queue.pop_front() {
        let next_num = ccc[x][y] + 1;
        for &(dx, dy) in dxdys.iter() {
            let xdx = (x as isize + dx) as usize;
            let ydy = (y as isize + dy) as usize;
            let dist_num = ccc[xdx][ydy];
            if dist_num == std::isize::MAX {
                println!("{}", next_num);
                return;
            }
            if dist_num == -1 {
                ccc[xdx][ydy] = next_num;
                queue.push_back((xdx, ydy));
            }
        }
    }
}
