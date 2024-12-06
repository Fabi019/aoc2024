use std::{collections::HashSet, hash::Hash};

#[derive(PartialEq, Eq)]
struct PosDir((usize, usize), (isize, isize));

impl Hash for PosDir {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(self.0.0 as u8);
        state.write_u8(self.0.1 as u8);
        state.write_u8(self.1.0 as u8);
        state.write_u8(self.1.1 as u8);
    }
}

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
    start @ (mut gx, mut gy): (usize, usize),
    idir @ (mut dx, mut dy): (isize, isize),
    (ox, oy): (usize, usize),
) -> Result<HashSet<PosDir>, ()> {
    let mut visited = HashSet::new();
    visited.insert(PosDir(start, idir));

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

        // Only store direction when simulating with additional obstacle
        if ox != usize::MAX && oy != usize::MAX {
            if !visited.insert(PosDir(np, (dx, dy))) {
                return Err(()); // Cycle detected
            }
        } else {
            visited.insert(PosDir(np, idir)); // always use same orientation
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
        .filter_map(|PosDir(pos, _)| simulate(&grid, start, (0, -1), pos).err())
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
