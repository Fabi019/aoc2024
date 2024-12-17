use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

aoc2024::main!("../../assets/day16.txt");

fn part1(input: &str) -> u32 {
    let map = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    dijkstra(&map, true)
}

fn dijkstra(grid: &[&[u8]], first_path: bool) -> u32 {
    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::new();

    let start = (1, grid.len() - 2);
    debug_assert!(grid[start.1][start.0] == b'S');

    let end = (grid[0].len() - 2, 1);
    debug_assert!(grid[end.1][end.0] == b'E');

    queue.push((Reverse(0), start, (1, 0), vec![start]));

    let mut min_cost = None;
    let mut places = HashSet::new();

    while let Some((Reverse(cost), pos @ (x, y), dir, path)) = queue.pop() {
        if cost > min_cost.unwrap_or(u32::MAX) {
            continue;
        }

        if pos == end {
            if first_path {
                return cost;
            }

            if let Some(min) = min_cost {
                if min == cost {
                    places.extend(path);
                }
            } else {
                min_cost = Some(cost);
            }

            continue;
        }

        let c = visited.entry((pos, dir)).or_insert(u32::MAX);
        if *c < cost {
            continue;
        }
        *c = cost;

        for d @ &(dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            // Prevent going back
            if (-dx, -dy) == dir {
                continue;
            }

            let x = x.wrapping_add_signed(dx);
            let y = y.wrapping_add_signed(dy);

            let c = grid[y][x];
            if c == b'#' {
                continue;
            }

            let cost = if *d != dir { cost + 1000 } else { cost };

            let mut path = path.clone();
            if !first_path {
                path.push((x, y));
            }

            queue.push((Reverse(cost + 1), (x, y), (dx, dy), path));
        }
    }

    if first_path {
        unreachable!("No path found");
    } else {
        places.len() as u32
    }
}

fn part2(input: &str) -> u32 {
    let map = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    dijkstra(&map, false)
}

aoc2024::test!(
    "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
",
    11048,
    64
);
