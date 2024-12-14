const WIDTH: i32 = if cfg!(test) { 11 } else { 101 };
const HEIGHT: i32 = if cfg!(test) { 7 } else { 103 };

aoc2024::main!("../../assets/day14.txt");

fn part1(input: &str) -> usize {
    let mut robots = input
        .lines()
        .map(|l| {
            let (pos, vel) = l.split_once(" ").unwrap();
            let (px, py) = pos[2..].split_once(",").unwrap();
            let (vx, vy) = vel[2..].split_once(",").unwrap();
            (
                (px.parse::<i32>().unwrap(), py.parse::<i32>().unwrap()),
                (vx.parse::<i32>().unwrap(), vy.parse::<i32>().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    let quads = cycle(&mut robots, 100, (WIDTH / 2, HEIGHT / 2));
    quads.into_iter().product()
}

#[inline(always)]
#[allow(clippy::type_complexity)]
fn cycle(robots: &mut [((i32, i32), (i32, i32))], step: i32, middle: (i32, i32)) -> [usize; 4] {
    let mut quads = [0; 4];
    for ((px, py), (vx, vy)) in robots.iter_mut() {
        *px = (*px + *vx * step) % WIDTH;
        *py = (*py + *vy * step) % HEIGHT;

        if *px < 0 {
            *px += WIDTH;
        }

        if *py < 0 {
            *py += HEIGHT;
        }

        if *px == middle.0 || *py == middle.1 {
            continue;
        }

        let quad = match (*px < middle.0, *py < middle.1) {
            (true, true) => 0,
            (false, true) => 1,
            (true, false) => 2,
            (false, false) => 3,
        };

        quads[quad] += 1;
    }
    quads
}

fn part2(input: &str) -> i32 {
    let mut robots = input
        .lines()
        .map(|l| {
            let (pos, vel) = l.split_once(" ").unwrap();
            let (px, py) = pos[2..].split_once(",").unwrap();
            let (vx, vy) = vel[2..].split_once(",").unwrap();
            (
                (px.parse::<i32>().unwrap(), py.parse::<i32>().unwrap()),
                (vx.parse::<i32>().unwrap(), vy.parse::<i32>().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    let total_robots = robots.len();

    for steps in 1..(WIDTH * HEIGHT) {
        let quads = cycle(&mut robots, 1, (WIDTH / 2, HEIGHT / 2));
        for quad in quads {
            if quad >= (total_robots as f64 / 2.2) as usize {
                // println!("{steps}");
                // for y in 0..HEIGHT {
                //     for x in 0..WIDTH {
                //         if robots.iter().any(|e| e.0 == (x, y)) {
                //             print!("X");
                //         } else {
                //             print!(".");
                //         }
                //     }
                //     println!();
                // }
                // println!();
                return steps;
            }
        }
    }

    0
}

aoc2024::test!(
    "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
",
    12,
    0
);
