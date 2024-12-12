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
    start: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    sides: bool,
) -> (usize, usize) {
    let mut area = HashSet::new();
    area.insert(start);

    let mut perimeter = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let search = grid[start.1][start.0];

    while let Some(pos @ (cx, cy)) = queue.pop_front() {
        for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let np @ (x, y) = (cx as isize + dx, cy as isize + dy);
            if x < 0 || y < 0 || y >= grid.len() as isize || x >= grid[0].len() as isize {
                perimeter.insert((pos, np));
                continue;
            }

            let c = grid[y as usize][x as usize];
            if c == search {
                if area.insert((x as usize, y as usize)) {
                    queue.push_back((x as usize, y as usize));
                }
            } else {
                perimeter.insert((pos, np));
            }
        }
    }

    visited.extend(&area);

    if sides {
        return (area.len(), count_sides(&perimeter));
    }

    (area.len(), perimeter.len())
}

fn count_sides(perimeter: &HashSet<((usize, usize), (isize, isize))>) -> usize {
    let inner = perimeter
        .iter()
        .map(|e| (e.0.0 as isize, e.0.1 as isize))
        .collect::<HashSet<(isize, isize)>>();
    let outer = perimeter
        .iter()
        .map(|e| e.1)
        .collect::<HashSet<(isize, isize)>>();

    if inner.len() == 1 {
        return 4; // single piece (4 edges)
    }

    let mut edges = 0;

    // find all convex edges
    for (x, y) in &inner {
        let offsets = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
        let left = outer.contains(&(x + offsets[0].0, y + offsets[0].1));
        let right = outer.contains(&(x + offsets[1].0, y + offsets[1].1));
        let top = outer.contains(&(x + offsets[2].0, y + offsets[2].1));
        let bottom = outer.contains(&(x + offsets[3].0, y + offsets[3].1));

        // cases with 1 edge
        if left && bottom && !top && !right {
            if !inner.contains(&(x - 1, y + 1)) {
                edges += 1;
            }
        } else if left && top && !bottom && !right {
            if !inner.contains(&(x - 1, y - 1)) {
                edges += 1;
            }
        } else if top && right && !bottom && !left {
            if !inner.contains(&(x + 1, y - 1)) {
                edges += 1;
            }
        } else if right && bottom && !left && !top {
            if !inner.contains(&(x + 1, y + 1)) {
                edges += 1;
            }
        }

        // cases with 2 edges
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
        let offsets = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
        let left = inner.contains(&(x + offsets[0].0, y + offsets[0].1));
        let right = inner.contains(&(x + offsets[1].0, y + offsets[1].1));
        let top = inner.contains(&(x + offsets[2].0, y + offsets[2].1));
        let bottom = inner.contains(&(x + offsets[3].0, y + offsets[3].1));

        // cases with 1 edge
        if left && bottom && !top && !right {
            edges += 1;
        } else if left && top && !bottom && !right {
            edges += 1;
        } else if top && right && !bottom && !left {
            edges += 1;
        } else if right && bottom && !left && !top {
            edges += 1;
        }

        // cases with 2 edges
        if left && top && right && !bottom {
            edges += 2;
        } else if top && right && bottom && !left {
            edges += 2;
        } else if right && bottom && left && !top {
            edges += 2;
        } else if bottom && left && top && !right {
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
