aoc2024::main!("../../assets/day04.txt");

fn part1(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let c = grid[y][x];
            if c != 'X' {
                continue;
            }

            for dy in -1..=1 {
                'outer: for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let mut cx = x as isize + dx;
                    let mut cy = y as isize + dy;

                    for i in "MAS".chars() {
                        if cx < 0
                            || cy < 0
                            || cx >= grid[0].len() as isize
                            || cy >= grid.len() as isize
                        {
                            continue 'outer;
                        }

                        let c = grid[cy as usize][cx as usize];
                        if c != i {
                            continue 'outer;
                        }

                        cx += dx;
                        cy += dy;
                    }

                    count += 1;
                }
            }
        }
    }

    count
}

fn part2(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;

    for y in 0..grid.len() {
        'outer: for x in 0..grid[0].len() {
            let c = grid[y][x];
            if c != 'A' {
                continue;
            }

            if x == 0 || x >= grid[0].len() - 1 || y == 0 || y >= grid.len() - 1 {
                continue;
            }

            for ((xa, ya), (xb, yb)) in [((-1, -1), (1, 1)), ((-1, 1), (1, -1))] {
                let (xa, ya) = (x as isize + xa, y as isize + ya);
                let (xb, yb) = (x as isize + xb, y as isize + yb);

                let first = grid[ya as usize][xa as usize];
                let second = grid[yb as usize][xb as usize];

                if !(first == 'S' && second == 'M' || first == 'M' && second == 'S') {
                    continue 'outer;
                }
            }

            count += 1;
        }
    }

    count
}

aoc2024::test!(
    "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
",
    18,
    9
);
