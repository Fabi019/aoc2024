use std::collections::{HashMap, HashSet};

aoc2024::main!("../../assets/day08.txt");

fn part1(input: &str) -> u32 {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut ants: HashMap<_, HashSet<_>> = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let c = grid[y][x];
            if c == b'.' {
                continue;
            }

            ants.entry(c).or_default().insert((x as isize, y as isize));
        }
    }

    find_antinodes(ants, (grid[0].len() as isize, grid.len() as isize), false) as u32
}

fn find_antinodes(
    antennas: HashMap<u8, HashSet<(isize, isize)>>,
    (width, height): (isize, isize),
    multiples: bool,
) -> usize {
    let mut antinodes = HashSet::new();

    for (_, locs) in antennas {
        if multiples {
            antinodes.extend(&locs);
        }

        for (i, l) in locs.iter().enumerate() {
            for ol in locs.iter().skip(i + 1) {
                let (dx, dy) = (l.0 - ol.0, l.1 - ol.1);

                for mult in 1.. {
                    let mut inserted = false;

                    let (x, y) = (l.0 + dx * mult, l.1 + dy * mult);
                    if !(x < 0 || y < 0 || y >= height || x >= width) {
                        antinodes.insert((x, y));
                        inserted = true;
                    }

                    let (x, y) = (ol.0 - dx * mult, ol.1 - dy * mult);
                    if !(x < 0 || y < 0 || y >= height || x >= width) {
                        antinodes.insert((x, y));
                        inserted = true;
                    }

                    if !inserted || !multiples {
                        break;
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn part2(input: &str) -> u32 {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut ants: HashMap<_, HashSet<_>> = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let c = grid[y][x];
            if c == b'.' {
                continue;
            }

            ants.entry(c).or_default().insert((x as isize, y as isize));
        }
    }

    find_antinodes(ants, (grid[0].len() as isize, grid.len() as isize), true) as u32
}

aoc2024::test!(
    "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
",
    14,
    34
);
