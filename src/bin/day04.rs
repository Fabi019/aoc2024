aoc2024::main!("../../assets/day04.txt");

fn part1(input: &str) -> u32 {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != b'X' {
                continue;
            }

            for dy in -1..=1 {
                if dy == -1 && y < 3 || dy == 1 && y >= grid.len() - 3 {
                    continue;
                }

                'next: for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    if dx == -1 && x < 3 || dx == 1 && x >= grid[0].len() - 3 {
                        continue;
                    }

                    let (mut cx, mut cy) = (x as isize + dx, y as isize + dy);

                    for &i in b"MAS" {
                        if grid[cy as usize][cx as usize] != i {
                            continue 'next;
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
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut count = 0;

    for y in 0..grid.len() {
        'next: for x in 0..grid[0].len() {
            if grid[y][x] != b'A' {
                continue;
            }

            if x == 0 || x >= grid[0].len() - 1 || y == 0 || y >= grid.len() - 1 {
                continue;
            }

            for ((xa, ya), (xb, yb)) in [((-1, -1), (1, 1)), ((-1, 1), (1, -1))] {
                let (xa, ya) = (x as isize + xa, y as isize + ya);
                let (xb, yb) = (x as isize + xb, y as isize + yb);

                let a = grid[ya as usize][xa as usize];
                let b = grid[yb as usize][xb as usize];

                if !(a == b'S' && b == b'M' || a == b'M' && b == b'S') {
                    continue 'next;
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
