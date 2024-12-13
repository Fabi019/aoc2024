aoc2024::main!("../../assets/day13.txt");

fn part1(input: &str) -> u32 {
    let machines = input.split("\n\n").map(|c| {
        fn parse(s: &str) -> (u32, u32) {
            let (x, y) = s.split_once(", ").unwrap();
            (x[2..].parse().unwrap(), y[2..].parse().unwrap())
        }
        let mut l = c.lines();
        let a = l.next().unwrap().split(": ").nth(1).unwrap();
        let b = l.next().unwrap().split(": ").nth(1).unwrap();
        let p = l.next().unwrap().split(": ").nth(1).unwrap();
        (parse(a), parse(b), parse(p))
    }).collect::<Vec<_>>();

    let mut min_tokens = 0;

    for ((ax, ay), (bx, by), (px, py)) in machines {
        let min_bx = px / bx;
        let min_by = py / by;

        let min = min_bx.min(min_by);

        for i in (0..=min).rev() {
            let mut current_x = i * bx;
            let mut current_y = i * by;

            let mut a_count = 0;
            while current_x <= px && current_y <= py {
                if current_x == px && current_y == py {
                    println!("Found min with a * {a_count}, b * {i}");
                    min_tokens += a_count * 3 + i;
                    break;
                }
                
                current_x += ax;
                current_y += ay;
                a_count += 1;
            }
        }
    }

    min_tokens
}

fn part2(input: &str) -> i64 {
    let machines = input.split("\n\n").map(|c| {
        fn parse(s: &str) -> (i64, i64) {
            let (x, y) = s.split_once(", ").unwrap();
            (x[2..].parse().unwrap(), y[2..].parse().unwrap())
        }
        let mut l = c.lines();
        let a = l.next().unwrap().split(": ").nth(1).unwrap();
        let b = l.next().unwrap().split(": ").nth(1).unwrap();
        let p = l.next().unwrap().split(": ").nth(1).unwrap();
        (parse(a), parse(b), parse(p))
    }).collect::<Vec<_>>();

    let mut min_tokens = 0;

    for ((ax, ay), (bx, by), (px, py)) in machines {
        let px = px + 10000000000000;
        let py = py + 10000000000000;

        let a = (px * by - bx * py) / (ax * by - bx * ay);
        let b = (px - ax * a) / bx;

        // validate solution
        if a * ax + b * bx != px || a * ay + b * by != py {
            continue;
        }

        min_tokens += a * 3 + b;
    }

    min_tokens
}

aoc2024::test!(
    "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
", 480, 875318608908
);
