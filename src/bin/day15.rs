aoc2024::main!("../../assets/day15.txt");

fn print_map(map: &[Vec<u8>], (rx, ry): (usize, usize)) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if y == ry && x == rx {
                print!("@");
            } else {
                print!("{}", map[y][x] as char);
            }
        }
        println!();
    }
    println!();
}

fn part1(input: &str) -> usize {
    let (map, instr) = input.split_once("\n\n").unwrap();
    let mut map = map
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let instr = instr.as_bytes();

    let (mut rx, mut ry) = (map[0].len() / 2 - 1, map.len() / 2 - 1);

    debug_assert!(map[ry][rx] == b'@');
    map[ry][rx] = b'.';

    'next: for ins in instr {
        let (dx, dy) = match ins {
            b'<' => (-1, 0),
            b'>' => (1, 0),
            b'^' => (0, -1),
            b'v' => (0, 1),
            _ => continue,
        };

        //println!("Move {}:", *ins as char);

        let (tx, ty) = (rx.wrapping_add_signed(dx), ry.wrapping_add_signed(dy));
        let i = map[ty][tx];

        if i == b'#' {
            continue;
        } else if i == b'O' {
            let (mut ax, mut ay) = (tx, ty);
            loop {
                ax = ax.wrapping_add_signed(dx);
                ay = ay.wrapping_add_signed(dy);

                let c = map[ay][ax];
                if c == b'#' {
                    continue 'next;
                } else if c == b'.' {
                    map[ty][tx] = b'.';
                    map[ay][ax] = b'O';
                    break;
                }
            }
        }

        rx = tx;
        ry = ty;

        //print_map(&map, (rx, ry));
    }

    let mut gps_sum = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == b'O' {
                gps_sum += y * 100 + x;
            }
        }
    }

    gps_sum
}

fn part2(input: &str) -> usize {
    let (map, instr) = input.split_once("\n\n").unwrap();
    let mut map = map
        .lines()
        .map(|l| {
            let mut line = Vec::new();
            for c in l.as_bytes() {
                let n = match c {
                    b'.' => b"..",
                    b'#' => b"##",
                    b'O' => b"[]",
                    b'@' => b"@.",
                    c => unreachable!("Invalid map symbol: {}", *c as char),
                };
                line.extend_from_slice(n);
            }
            line
        })
        .collect::<Vec<_>>();
    let instr = instr.as_bytes();

    let (mut rx, mut ry) = (map[0].len() / 2 - 2, map.len() / 2 - 1);

    debug_assert!(map[ry][rx] == b'@');
    map[ry][rx] = b'.';

    //print_map(&map, (rx, ry));

    'next: for ins in instr {
        let (dx, dy) = match ins {
            b'<' => (-1, 0),
            b'>' => (1, 0),
            b'^' => (0, -1),
            b'v' => (0, 1),
            _ => continue,
        };

        let (tx, ty) = (rx.wrapping_add_signed(dx), ry.wrapping_add_signed(dy));
        let i = map[ty][tx];

        if i == b'#' {
            continue;
        } else if i == b'[' || i == b']' {
            if dy == 0 {
                let mut ax = tx;
                loop {
                    ax = ax.wrapping_add_signed(dx);

                    let c = map[ty][ax];
                    if c == b'#' {
                        continue 'next;
                    } else if c == b'.' {
                        map[ty].remove(ax);
                        map[ty].insert(tx, b'.');
                        break;
                    }
                }
            } else {
                let mut obstacles = Vec::new();

                // initialize with first element
                obstacles.push((tx, ty));
                if i == b'[' {
                    obstacles.push((tx + 1, ty));
                } else {
                    obstacles.push((tx - 1, ty));
                }

                loop {
                    let mut modified = false;
                    for i in 0..obstacles.len() {
                        let (px, py) = obstacles[i];
                        let ay = py.wrapping_add_signed(dy); // check whats behind
                        let c = map[ay][px];
                        if c == b'#' {
                            continue 'next; // impossible to push
                        } else if c == b'[' && !obstacles.contains(&(px, ay)) {
                            obstacles.push((px, ay));
                            obstacles.push((px + 1, ay));
                            modified = true;
                        } else if c == b']' && !obstacles.contains(&(px, ay)) {
                            obstacles.push((px, ay));
                            obstacles.push((px - 1, ay));
                            modified = true;
                        }
                    }

                    if !modified {
                        break;
                    }
                }

                // all should be possible to move up, start from back
                for (px, py) in obstacles.into_iter().rev() {
                    let c = map[py][px];
                    map[py.wrapping_add_signed(dy)][px] = c;
                    map[py][px] = b'.';
                }
            }
        }

        rx = tx;
        ry = ty;

        //print_map(&map, (rx, ry));
    }

    let mut gps_sum = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == b'[' {
                gps_sum += y * 100 + x;
            }
        }
    }

    gps_sum
}

aoc2024::test!(
    "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
",
    10092,
    9021
);
