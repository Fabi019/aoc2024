const WIDTH: i32 = if cfg!(test) { 11 } else { 101 };
const HEIGHT: i32 = if cfg!(test) { 7 } else { 103 };

aoc2024::main!("../../assets/day14.txt");

fn part1(input: &str) -> usize {

    let robots = input.lines().map(|l| {
        let (pos, vel) = l.split_once(" ").unwrap();
        let (px, py) = pos[2..].split_once(",").unwrap();
        let (vx, vy) = vel[2..].split_once(",").unwrap();
        ((px.parse::<i32>().unwrap(), py.parse::<i32>().unwrap()), (vx.parse::<i32>().unwrap(), vy.parse::<i32>().unwrap()))
    });

    let mut positions = Vec::new();
    let steps = 100;
    
    for ((px, py), (vx, vy)) in robots {
        let mut x = (px + vx * steps) % WIDTH;
        let mut y = (py + vy * steps) % HEIGHT;

        if x < 0 {
            x += WIDTH;
        }

        if y < 0 {
            y += HEIGHT;
        }

        positions.push((x, y));
    }

    let middle_x = WIDTH / 2;
    let middle_y = HEIGHT / 2;

    let mut quad = [0; 4];
    quad[0] = positions.iter().filter(|&p| p.0 < middle_x && p.1 < middle_y).count();
    quad[1] = positions.iter().filter(|&p| p.0 > middle_x && p.1 < middle_y).count();
    quad[2] = positions.iter().filter(|&p| p.0 < middle_x && p.1 > middle_y).count();
    quad[3] = positions.iter().filter(|&p| p.0 > middle_x && p.1 > middle_y).count();

    quad[0] * quad[1] * quad[2] * quad[3]
}

fn part2(input: &str) -> usize {
    let mut robots = input.lines().map(|l| {
        let (pos, vel) = l.split_once(" ").unwrap();
        let (px, py) = pos[2..].split_once(",").unwrap();
        let (vx, vy) = vel[2..].split_once(",").unwrap();
        ((px.parse::<i32>().unwrap(), py.parse::<i32>().unwrap()), (vx.parse::<i32>().unwrap(), vy.parse::<i32>().unwrap()))
    }).collect::<Vec<_>>();

    let total_robots = robots.len();

    for steps in 1..(WIDTH * HEIGHT) {
        for ((px, py), (vx, vy)) in robots.iter_mut() {
            *px = (*px + *vx) % WIDTH;
            *py = (*py + *vy) % HEIGHT;

            if *px < 0 {
                *px += WIDTH;
            }

            if *py < 0 {
                *py += HEIGHT;
            }
        }

        let middle_x = WIDTH / 2;
        let middle_y = HEIGHT / 2;
    
        let mut quads = [0; 4];
        quads[0] = robots.iter().filter(|(p, _)| p.0 < middle_x && p.1 < middle_y).count();
        quads[1] = robots.iter().filter(|(p, _)| p.0 > middle_x && p.1 < middle_y).count();
        quads[2] = robots.iter().filter(|(p, _)| p.0 < middle_x && p.1 > middle_y).count();
        quads[3] = robots.iter().filter(|(p, _)| p.0 > middle_x && p.1 > middle_y).count();

        for quad in quads {
            if quad >= (total_robots as f64 / 2.2) as usize {
                println!("{steps}");
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        if robots.iter().any(|e| e.0 == (x, y)) {
                            print!("X");
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
                println!();
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
", 12, 0
);
