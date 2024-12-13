aoc2024::main!("../../assets/day13.txt");

fn part1(input: &str) -> i64 {
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
    });

    machines.fold(0, |acc, ((ax, ay), (bx, by), (px, py))| {
        let a = (px * by - bx * py) / (ax * by - bx * ay);
        let b = (px - ax * a) / bx;

        // validate solution
        if a * ax + b * bx != px || a * ay + b * by != py {
            return acc;
        }

        acc + a * 3 + b
    })
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
    });

    machines.fold(0, |acc, ((ax, ay), (bx, by), (px, py))| {
        let px = px + 10000000000000;
        let py = py + 10000000000000;

        let a = (px * by - bx * py) / (ax * by - bx * ay);
        let b = (px - ax * a) / bx;

        // validate solution
        if a * ax + b * bx != px || a * ay + b * by != py {
            return acc;
        }

        acc + a * 3 + b
    })
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
",
    480,
    875318608908
);
