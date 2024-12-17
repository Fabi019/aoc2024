aoc2024::main!("../../assets/day17.txt");

fn part1(input: &str) -> String {
    let (register, program) = input.split_once("\n\n").unwrap();
    let mut register = register.lines().map(|l| l[12..].parse::<u64>().unwrap());
    let program = program.lines().next().unwrap()[9..]
        .split(",")
        .map(|c| c.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut r_a = register.next().unwrap();
    let mut r_b = register.next().unwrap();
    let mut r_c = register.next().unwrap();

    let mut r_ip = 0;

    let mut output = Vec::new();

    loop {
        if let Some(out) = cycle(&program, &mut r_ip, &mut r_a, &mut r_b, &mut r_c) {
            output.push(out.to_string());
        }

        if r_ip >= program.len() {
            break;
        }
    }

    output.join(",")
}

#[inline(always)]
fn combo_op(op: u64, r_a: u64, r_b: u64, r_c: u64) -> u64 {
    match op {
        l @ 0..=3 => l,
        4 => r_a,
        5 => r_b,
        6 => r_c,
        op => unreachable!("Invalid combo op {op}"),
    }
}

fn cycle(
    program: &[u64],
    r_ip: &mut usize,
    r_a: &mut u64,
    r_b: &mut u64,
    r_c: &mut u64,
) -> Option<u64> {
    let mut output = None;

    let instr = program[*r_ip];
    let op = program[*r_ip + 1];

    match instr {
        // adv: division
        0 => {
            let op = combo_op(op, *r_a, *r_b, *r_c);
            *r_a /= 2u64.pow(op as u32);
        }

        // bxl: xor
        1 => *r_b ^= op,

        // bst
        2 => {
            let op = combo_op(op, *r_a, *r_b, *r_c);
            *r_b = op % 8;
        }

        // jnz: jump not zero
        3 => {
            if *r_a != 0 {
                *r_ip = op as usize;
                return None;
            }
        }

        // bxc
        4 => *r_b ^= *r_c,

        // out
        5 => {
            let op = combo_op(op, *r_a, *r_b, *r_c) % 8;
            let _ = output.insert(op);
        }

        // bdv
        6 => {
            let op = combo_op(op, *r_a, *r_b, *r_c);
            *r_b = *r_a / 2u64.pow(op as u32);
        }

        // cdv
        7 => {
            let op = combo_op(op, *r_a, *r_b, *r_c);
            *r_c = *r_a / 2u64.pow(op as u32);
        }

        i => unreachable!("Illegal instruction {i} at {r_ip}"),
    }

    *r_ip += 2;
    output
}

fn part2(input: &str) -> u64 {
    let (_register, program) = input.split_once("\n\n").unwrap();
    let program = program.lines().next().unwrap()[9..]
        .split(",")
        .map(|c| c.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut a = 0;

    // check each digit going from the back
    for i in 0..program.len() {
        let mut na = a * 8;

        // loops through every possible a%8 (a+0, .., a+7)
        loop {
            let mut out_index = program.len() - i - 1; // compare only from out_index..end
            let mut r_a = na;

            // A will be /= 8 in every run
            'outer: while r_a != 0 {
                let mut r_ip = 0;
                let mut r_b = 0;
                let mut r_c = 0;

                loop {
                    if let Some(out) = cycle(&program, &mut r_ip, &mut r_a, &mut r_b, &mut r_c) {
                        // check if digits match
                        if out_index < program.len() && out == program[out_index] {
                            out_index += 1;
                            break; // rerun until a == 0
                        } else {
                            break 'outer; // check a+1
                        }
                    }
                }
            }

            // check if all digits were validated
            if out_index == program.len() {
                a = na;
                break;
            }

            na += 1;
        }
    }

    a
}

aoc2024::test!(
    "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
",
    "4,6,3,5,6,3,5,2,1,0",
    "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
",
    117440
);
