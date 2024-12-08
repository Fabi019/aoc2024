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

    let mut antinodes = HashSet::new();

    for (_, locs) in ants {

        for (i, l) in locs.iter().enumerate() {
            for ol in locs.iter().skip(i + 1) {
                let (dx, dy) = (l.0 - ol.0, l.1 - ol.1);

                let x = l.0 + dx;
                let y = l.1 + dy;
                
                if !(x < 0 || y < 0 || y >= grid.len() as isize || x >= grid[0].len() as isize) {
                    antinodes.insert((x, y));
                }

                let x = ol.0 - dx;
                let y = ol.1 - dy;
                
                if !(x < 0 || y < 0 || y >= grid.len() as isize || x >= grid[0].len() as isize) {
                    antinodes.insert((x, y));
                }
            }
        }
    }

    antinodes.len() as u32
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

    let mut antinodes = HashSet::new();

    for (_, locs) in ants {
        for (i, l) in locs.iter().enumerate() {
            antinodes.insert(*l);

            for ol in locs.iter().skip(i + 1) {
                antinodes.insert(*ol);

                let (dx, dy) = (l.0 - ol.0, l.1 - ol.1);

                for mult in 1.. {
                    let mut inserted = false;

                    let x = l.0 + dx * mult;
                    let y = l.1 + dy * mult;
                    
                    if !(x < 0 || y < 0 || y >= grid.len() as isize || x >= grid[0].len() as isize) {
                        antinodes.insert((x, y));
                        inserted = true;
                    }

                    let x = ol.0 - dx * mult;
                    let y = ol.1 - dy * mult;
                    
                    if !(x < 0 || y < 0 || y >= grid.len() as isize || x >= grid[0].len() as isize) {
                        antinodes.insert((x, y));
                        inserted = true;
                    }

                    if !inserted {
                        break;
                    }
                }
            }
        }
    }

    antinodes.len() as u32
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
", 14, 34
);
