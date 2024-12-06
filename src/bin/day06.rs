use std::collections::{HashMap, HashSet};

type PosDirMap = HashMap<(usize, usize), HashSet<(isize, isize)>>;

aoc2024::main!("../../assets/day06.txt");

fn part1(input: &str) -> u32 {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut start = (0, 0);

    'outer: for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == b'^' {
                start = (x, y);
                break 'outer;
            }
        }
    }

    simulate(&grid, start, (0, -1), (usize::MAX, usize::MAX))
        .unwrap()
        .len() as u32
}

fn simulate(
    grid: &[&[u8]],
    (mut gx, mut gy): (usize, usize),
    (mut dx, mut dy): (isize, isize),
    (ox, oy): (usize, usize),
) -> Result<PosDirMap, ()> {
    let mut visited = HashMap::new();
    visited.insert((gx, gy), HashSet::from_iter([(dx, dy)]));

    loop {
        let (nx, ny) = (gx as isize + dx, gy as isize + dy);
        if nx < 0 || ny < 0 || nx >= grid[0].len() as isize || ny >= grid.len() as isize {
            break;
        }

        let np @ (nx, ny) = (nx as usize, ny as usize);
        if grid[ny][nx] == b'#' || nx == ox && ny == oy {
            // Turn 90 deg cw
            let tmp = dx;
            dx = -dy;
            dy = tmp;
            continue;
        }

        let entry = visited.entry(np).or_insert(HashSet::new());
        if !entry.insert((dx, dy)) {
            return Err(()); // Cycle detected
        }

        gx = nx;
        gy = ny;
    }

    Ok(visited)
}

fn part2(input: &str) -> u32 {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut start = (0, 0);

    'outer: for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == b'^' {
                start = (x, y);
                break 'outer;
            }
        }
    }

    simulate(&grid, start, (0, -1), (usize::MAX, usize::MAX))
        .unwrap()
        .into_iter()
        .filter_map(|(pos, _)| simulate(&grid, start, (0, -1), pos).err())
        .count() as u32
}

aoc2024::test!(
    "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
    41,
    6
);
