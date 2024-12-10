use std::collections::{HashSet, VecDeque};

aoc2024::main!("../../assets/day10.txt");

fn part1(input: &str) -> u32 {
    let map = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let mut starts = vec![];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == b'0' {
                starts.push((x as isize, y as isize));
            }
        }
    }

    let mut total_score = 0;

    for start in starts {
        total_score += find_paths(&map, start, true);
    }

    total_score
}

fn find_paths(map: &[&[u8]], start: (isize, isize), unique: bool) -> u32 {
    let mut visited = HashSet::new();
    let mut count = 0;

    let mut queue: VecDeque<((isize, isize), u32)> = VecDeque::new();
    queue.push_back((start, 0));

    while let Some(((cx, cy), height)) = queue.pop_front() {
        for (dx, dy) in &[(0, 1), (-1, 0), (1, 0), (0, -1)] {
            let (x, y): (isize, isize) = (cx + dx, cy + dy);
            if x < 0 || y < 0 || x >= map[0].len() as isize || y >= map.len() as isize {
                continue;
            }

            if let Some(h) = (map[y as usize][x as usize] as char).to_digit(10) {
                if h == height + 1 {
                    if h == 9 && (!unique || visited.insert((x, y))) {
                        count += 1;
                        continue;
                    }
                    queue.push_back(((x, y), h));
                }
            }
        }
    }

    count
}

fn part2(input: &str) -> u32 {
    let map = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let mut starts = vec![];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == b'0' {
                starts.push((x as isize, y as isize));
            }
        }
    }

    let mut total_score = 0;

    for start in starts {
        total_score += find_paths(&map, start, false);
    }

    total_score
}

aoc2024::test!(
    "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
",
    36,
    81
);
