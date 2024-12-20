use std::collections::HashMap;
use std::collections::hash_map::Entry::Vacant;

aoc2024::main!("../../assets/day20.txt");

fn part1(input: &str) -> usize {
    let map = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            match map[y][x] {
                b'S' => start = (x, y),
                b'E' => end = (x, y),
                _ => {}
            }
        }
    }

    let path = find_path(&map, start, end);
    let mut shortcuts = vec![];

    for (&(x, y), steps) in &path {
        for &(dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            if dx < 0 && x <= 1 || dy < 0 && y <= 1 {
                continue;
            }

            let x = x.wrapping_add_signed(dx * 2);
            let y = y.wrapping_add_signed(dy * 2);

            if let Some(other) = path.get(&(x, y)) {
                if other <= steps || other - steps - 2 < 100 {
                    continue; // no shortcut
                }

                shortcuts.push(other - steps - 2);
            }
        }
    }

    shortcuts.len()
}

fn find_path(
    grid: &[&[u8]],
    mut start: (usize, usize),
    end: (usize, usize),
) -> HashMap<(usize, usize), u32> {
    let mut path: HashMap<(usize, usize), u32> = HashMap::new();
    path.insert(start, 0);

    for step in 1.. {
        if start == end {
            break;
        }

        for &(dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let x = start.0.wrapping_add_signed(dx);
            let y = start.1.wrapping_add_signed(dy);

            let c = grid[y][x];
            if c == b'#' {
                continue;
            } else if let Vacant(e) = path.entry((x, y)) {
                e.insert(step);
                start = (x, y);
                break;
            }
        }
    }

    path
}

fn part2(input: &str) -> usize {
    let map = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            match map[y][x] {
                b'S' => start = (x, y),
                b'E' => end = (x, y),
                _ => {}
            }
        }
    }

    let path = find_path(&map, start, end);

    let mut shortcuts = vec![];

    for (&(x, y), steps) in &path {
        // top left corner
        let sx = if x <= 20 {
            0
        } else {
            x.wrapping_add_signed(-20)
        };
        let sy = if y <= 20 {
            0
        } else {
            y.wrapping_add_signed(-20)
        };
        // bottom right
        let ex = x + 20;
        let ey = y + 20;

        for cy in sy..=ey {
            for cx in sx..=ex {
                let dst = (x.abs_diff(cx) + y.abs_diff(cy)) as u32;
                if dst > 20 {
                    continue; // maximum of 20 steps
                }

                if let Some(other) = path.get(&(cx, cy)) {
                    if other <= steps || other - steps - dst < 100 {
                        continue; // no shortcut
                    }

                    shortcuts.push(other - steps - dst);
                }
            }
        }
    }

    shortcuts.len()
}

aoc2024::test!(
    "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
",
    0,
    0
);
