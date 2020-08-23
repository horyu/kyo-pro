#![allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;
// use itertools::Itertools;
use proconio::{input, marker::*};

type Point = (usize, usize);

fn main() {
    input! {
        h: usize,
        w: usize,
        ch: Usize1,
        cw: Usize1,
        dh: Usize1,
        dw: Usize1,
        sss: [Chars; h],
    };

    let (spaces, point_to_space_index) = make_spaces(h, w, &sss);

    if spaces.len() == 1 {
        println!("0");
        return;
    }
    let c_index = *point_to_space_index.get(&(cw, ch)).unwrap();
    let d_index = *point_to_space_index.get(&(dw, dh)).unwrap();

    println!(
        "{}",
        search_count(h, w, spaces, c_index, d_index, point_to_space_index)
    );
}

fn make_spaces(
    h: usize,
    w: usize,
    sss: &Vec<Vec<char>>,
) -> (Vec<Vec<Point>>, HashMap<Point, usize>) {
    let mut spaces: Vec<Vec<Point>> = vec![];
    // space = vec![(0, 0), (0, 1), (0, 2), (1, 2)];
    let mut point_to_space_index: HashMap<Point, usize> = HashMap::new();
    let mut viewed_points: HashSet<Point> = HashSet::new();
    let mut index = 0usize;

    for y in 0usize..h {
        for x in 0usize..w {
            let point: Point = (x, y);
            if viewed_points.contains(&point) {
                continue;
            } else {
                viewed_points.insert(point);
            }
            if sss[y][x] == '.' {
                // 新規のスペースを発見
                let mut space: Vec<Point> = vec![point];
                let space_index = index;
                point_to_space_index.insert(point, space_index);
                let mut queue: VecDeque<Point> = VecDeque::new();
                queue.push_back(point);
                while let Some((qpx, qpy)) = queue.pop_front() {
                    let check_points: Vec<Point> = {
                        let mut tmp: Vec<Point> = vec![];
                        if qpx > 0 {
                            tmp.push((qpx - 1, qpy));
                        }
                        if qpx < w - 1 {
                            tmp.push((qpx + 1, qpy));
                        }
                        if qpy > 0 {
                            tmp.push((qpx, qpy - 1));
                        }
                        if qpy < h - 1 {
                            tmp.push((qpx, qpy + 1));
                        }
                        tmp
                    };
                    for check_point in check_points {
                        if viewed_points.contains(&check_point) {
                            continue;
                        } else {
                            viewed_points.insert(check_point);
                            let (cx, cy) = check_point;
                            if sss[cy][cx] == '.' {
                                point_to_space_index.insert(check_point, space_index);
                                space.push(check_point);
                                queue.push_back(check_point);
                            }
                        }
                    }
                }
                spaces.push(space);
                index += 1;
            }
        }
    }
    return (spaces, point_to_space_index);
}

fn search_count(
    h: usize,
    w: usize,
    spaces: Vec<Vec<Point>>,
    c_index: usize,
    d_index: usize,
    point_to_space_index: HashMap<Point, usize>,
) -> isize {
    let warpable_areas: Vec<HashSet<Point>> = spaces
        .iter()
        .map(|space| {
            let mut warpable_area = HashSet::new();
            let inner: HashSet<&Point> = HashSet::from_iter(space.iter());
            for &(x, y) in space {
                let up_y = std::cmp::max(y as isize - 2, 0) as usize;
                let down_y = std::cmp::min(y as isize + 2, h as isize - 1) as usize;
                for y in up_y..=down_y {
                    let left_x = std::cmp::max(x as isize - 2, 0) as usize;
                    let right_x = std::cmp::min(x as isize + 2, w as isize - 1) as usize;
                    for x in left_x..=right_x {
                        let p = (x, y);
                        if !inner.contains(&p) {
                            warpable_area.insert(p);
                        }
                    }
                }
            }
            return warpable_area;
        })
        .collect();

    let mut viewed_index = HashSet::new();
    viewed_index.insert(c_index);
    let mut queue: VecDeque<Vec<usize>> = VecDeque::new();
    queue.push_back(vec![c_index]);
    let mut count = 1;
    while let Some(indexes) = queue.pop_front() {
        let mut next_indexes = vec![];
        for index in indexes {
            let warpable_area = &warpable_areas[index];
            for &point in warpable_area {
                if let Some(&warpable_index) = point_to_space_index.get(&point) {
                    if warpable_index == d_index {
                        return count;
                    }
                    if !viewed_index.contains(&warpable_index) {
                        viewed_index.insert(warpable_index);
                        next_indexes.push(warpable_index);
                    }
                }
            }
        }
        count += 1;
        if !next_indexes.is_empty() {
            queue.push_back(next_indexes);
        }
    }
    return -1;
}
