use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

const SIZE: usize = if cfg!(test) { 7 } else { 71 };

aoc2024::main!("../../assets/day18.txt");

fn part1(input: &str) -> u32 {
    let mut grid = vec![vec![false; SIZE]; SIZE];

    let take = if cfg!(test) { 12 } else { 1024 };

    for l in input.lines().take(take) {
        let (x, y) = l.split_once(",").unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        grid[y][x] = true;
    }

    bfs(&grid)
}

fn bfs(map: &[Vec<bool>]) -> u32 {
    let start = (0, 0);
    let end = (SIZE - 1, SIZE - 1);

    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start));

    while let Some((Reverse(steps), pos @ (x, y))) = queue.pop() {
        if pos == end {
            return steps;
        }

        for &(dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (nx, ny) = (x as isize + dx, y as isize + dy);

            if nx < 0 || nx >= SIZE as isize || ny < 0 || ny >= SIZE as isize {
                continue;
            }

            let pos @ (nx, ny) = (nx as usize, ny as usize);

            if !map[ny][nx] && visited.insert(pos) {
                queue.push((Reverse(steps + 1), pos));
            }
        }
    }

    u32::MAX
}

fn part2(input: &str) -> (usize, usize) {
    let mut grid = vec![vec![false; SIZE]; SIZE];

    let take = if cfg!(test) { 12 } else { 1024 };
    let lines = input.lines().map(|l| {
        let (x, y) = l.split_once(",").unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        (x, y)
    });

    // add previous
    for (x, y) in lines.clone().take(take) {
        grid[y][x] = true;
    }

    for (x, y) in lines.skip(take) {
        grid[y][x] = true;
        if bfs(&grid) == u32::MAX {
            return (x, y);
        }
    }

    unreachable!("All paths succeeded!");
}

aoc2024::test!(
    "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
",
    22,
    (6, 1)
);
