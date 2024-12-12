use std::collections::{HashSet, VecDeque};

aoc2024::main!("../../assets/day12.txt");

fn part1(input: &str) -> usize {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut visited = HashSet::new();
    let mut total_price = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if visited.contains(&(x, y)) {
                continue;
            }

            let (area, perimeter) = explore_plot(&grid, (x, y), &mut visited, false);

            total_price += area * perimeter;
        }
    }

    total_price
}

fn explore_plot(
    grid: &[&[u8]],
    start @ (sx, sy): (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    sides: bool,
) -> (usize, usize) {
    let mut area = 1;
    visited.insert(start);

    let mut pcount = 0;
    let mut perimeter = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let search = grid[sy][sx];

    while let Some(pos @ (cx, cy)) = queue.pop_front() {
        for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let np @ (x, y) = (cx as isize + dx, cy as isize + dy);
            if x < 0 || y < 0 || y >= grid.len() as isize || x >= grid[0].len() as isize {
                if sides {
                    perimeter.insert((pos, np));
                } else {
                    pcount += 1;
                }
                continue;
            }
            let npos @ (x, y) = (x as usize, y as usize);
            let c = grid[y][x];
            if c == search {
                if visited.insert(npos) {
                    area += 1;
                    queue.push_back(npos);
                }
            } else if sides {
                perimeter.insert((pos, np));
            } else {
                pcount += 1;
            }
        }
    }

    if sides {
        return (area, count_sides(&perimeter));
    }

    (area, pcount)
}

#[allow(clippy::nonminimal_bool)]
#[allow(clippy::type_complexity)]
fn count_sides(perimeter: &HashSet<((usize, usize), (isize, isize))>) -> usize {
    let inner = perimeter
        .iter()
        .map(|e| (e.0.0 as isize, e.0.1 as isize))
        .collect::<HashSet<(isize, isize)>>();
    let outer = perimeter
        .iter()
        .map(|e| e.1)
        .collect::<HashSet<(isize, isize)>>();

    if inner.len() == 1 || inner.len() == 2 {
        return 4; // always has 4 edges
    }

    let mut edges = 0;

    fn get_nbs(x: isize, y: isize, set: &HashSet<(isize, isize)>) -> (bool, bool, bool, bool) {
        (
            set.contains(&(x - 1, y)), // left
            set.contains(&(x + 1, y)), // right
            set.contains(&(x, y - 1)), // top
            set.contains(&(x, y + 1)), // bottom
        )
    }

    // find all convex edges
    for (x, y) in &inner {
        let (left, right, top, bottom) = get_nbs(*x, *y, &outer);

        // cases with 1 edge
        if left && bottom && !top && !right && !inner.contains(&(x - 1, y + 1))
            || left && top && !bottom && !right && !inner.contains(&(x - 1, y - 1))
            || top && right && !bottom && !left && !inner.contains(&(x + 1, y - 1))
            || right && bottom && !left && !top && !inner.contains(&(x + 1, y + 1))
        {
            edges += 1;
        }

        // cases with 2 edges (check diagonal to not count double with concave edges)
        if left && top && right && !bottom {
            let lt = inner.contains(&(x - 1, y - 1));
            let rt = inner.contains(&(x + 1, y - 1));
            if !lt && !rt {
                edges += 2;
            } else if lt && !rt || rt && !lt {
                edges += 1;
            }
        } else if top && right && bottom && !left {
            let tr = inner.contains(&(x + 1, y - 1));
            let br = inner.contains(&(x + 1, y + 1));
            if !tr && !br {
                edges += 2;
            } else if tr && !br || br && !tr {
                edges += 1;
            }
        } else if right && bottom && left && !top {
            let rb = inner.contains(&(x + 1, y + 1));
            let lb = inner.contains(&(x - 1, y + 1));
            if !rb && !lb {
                edges += 2;
            } else if rb && !lb || lb && !rb {
                edges += 1;
            }
        } else if bottom && left && top && !right {
            let bl = inner.contains(&(x - 1, y + 1));
            let tl = inner.contains(&(x - 1, y - 1));
            if !bl && !tl {
                edges += 2;
            } else if bl && !tl || tl && !bl {
                edges += 1;
            }
        }
    }

    // find all concave edges
    for (x, y) in &outer {
        let (left, right, top, bottom) = get_nbs(*x, *y, &inner);

        // cases with 1 edge
        if left && bottom && !top && !right
            || top && right && !bottom && !left
            || right && bottom && !left && !top
            || left && top && !bottom && !right
        {
            edges += 1;
        }

        // cases with 2 edges
        if left && top && right && !bottom
            || top && right && bottom && !left
            || right && bottom && left && !top
            || bottom && left && top && !right
        {
            edges += 2;
        }

        // case with 4 edges
        if left && top && right && bottom {
            edges += 4;
        }
    }

    edges
}

fn part2(input: &str) -> usize {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut visited = HashSet::new();
    let mut total_price = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if visited.contains(&(x, y)) {
                continue;
            }

            let (area, perimeter) = explore_plot(&grid, (x, y), &mut visited, true);

            total_price += area * perimeter;
        }
    }

    total_price
}

aoc2024::test!(
    "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
    1930,
    1206
);
