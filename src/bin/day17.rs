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
            println!("ADV {op}: A = A / 2^{op} = {r_a}");
        }

        // bxl: xor
        1 => {
            *r_b ^= op;
            println!("BXL {op}: B = B ^ {op} = {r_b}");
        }

        // bst
        2 => {
            let op = combo_op(op, *r_a, *r_b, *r_c);
            *r_b = op % 8;
            println!("BST {op}: B = {op} % 8 = {r_b}");
        }

        // jnz: jump not zero
        3 => {
            println!("JNZ {op}: A = {r_a}");
            if *r_a != 0 {
                *r_ip = op as usize;
                return None;
            }
        }

        // bxc
        4 => {
            *r_b ^= *r_c;
            println!("BXC {op}: B = B ^ C = {r_b} ({r_c})");
        }

        // out
        5 => {
            let op = combo_op(op, *r_a, *r_b, *r_c) % 8;
            let _ = output.insert(op);
            println!("OUT {op}");
        }

        // bdv
        6 => {
            let op = combo_op(op, *r_a, *r_b, *r_c);
            *r_b = *r_a / 2u64.pow(op as u32);
            println!("BDV {op}: B = A / 2^{op}");
        }

        // cdv
        7 => {
            let op = combo_op(op, *r_a, *r_b, *r_c);
            *r_c = *r_a / 2u64.pow(op as u32);
            println!("CDV {op}: C = A / 2^{op}");
        }

        i => unreachable!("Illegal instruction {i} at IP={r_ip}"),
    }

    *r_ip += 2;
    output
}

fn part2(input: &str) -> u64 {
    let (register, program) = input.split_once("\n\n").unwrap();
    let mut register = register.lines().map(|l| l[12..].parse::<u64>().unwrap());
    let program = program.lines().next().unwrap()[9..]
        .split(",")
        .map(|c| c.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // BST 4	; B = A % 8
    // BXL 1	; B = B ^ 1     ; +- 1 if even/odd
    // CDV 5	; C = A / 2**B
    // BXL 5	; B = B ^ 5
    // BXC 5	; B = B ^ C
    // ADV 3	; A = A / 2**3 = A / 8
    // OUT 5	; B % 8
    // JNZ 0	; END if A=0 otherwise jump to top

    let _ = register.next().unwrap();
    let r_b = register.next().unwrap();
    let r_c = register.next().unwrap();

    let mut r_a = 8u64.pow(15);

    let mut out_index = program.len() - 1;

    let mut r_a = 0;
    for i in 0..program.len() {
        let mut na = r_a * 8;
        loop {
            let mut ta = na;
            let mut out_index = program.len() - i - 1;

            while ta != 0 {
                let mut b = ta % 8;     // BST 4
                b ^= 1;     // BXL 1
                let c = ta / 2u64.pow(b as u32);    // CDV 5
                b ^= 5;     // BXL 5
                b ^= c;     // BXC 5
        
                if out_index < program.len() && b % 8 == program[out_index] {
                    out_index += 1;
                    ta /= 8;
                } else {
                    break;
                }
            }

            if out_index == program.len() {
                r_a = na;
                break;
            }

            na += 1;
        }
    }

    return r_a;

    // loop {
    //     let ca = a * 8;

    //     while ca != 0 {
    //         let mut b = r_a % 8;
    //         b ^= 1;
    //         let c = ca / 2u64.pow(r_b as u32);
    //         b ^= 5;
    //         b ^= c;
    
    //         if b % 8 == program[out_index] {
    //             r_a *= 8;
    //         } else {
    //             r_a += 1;
    //             break;
    //         }

    //         out_index -= 1;
    //         ca *= 8;
    //     }
        
    //     if out_index == 0 {
    //         return r_a;
    //     }
    //     out_index = out_index.wrapping_sub(1);
    // }


    // loop {
    //     b = r_a % 8;
    //     b ^= 1;
    //     c = r_a / 2u64.pow(r_b as u32);
    //     b ^= 5;
    //     b ^= c;

    //     if b % 8 == program[out_index] {
    //         r_a *= 8;
    //     } else {
    //         r_a += 1;
    //         out_index = program.len() - 1;
    //         continue;
    //     }
        
    //     if out_index == 0 {
    //         return r_a;
    //     }
    //     out_index = out_index.wrapping_sub(1);
    // }

    // let start = 35215500000000; //8u64.pow(15);
    // let end = 8u64.pow(16);
    // 'next: for a in start..end {
    //     let mut r_a = a;
    //     let mut r_b = r_b;
    //     let mut r_c = r_c;

    //     let mut r_ip = 0;

    //     let mut out_index = 0;

    //     if a % 200_000_000 == 0 {
    //         println!("At {a}.. {}", ((a - start) as f64 / end as f64) * 100.0);
    //     }

    //     loop {
    //         r_b = r_a % 8;
    //         r_b = r_b ^ 1;
    //         r_c = r_a / 2u64.pow(r_b as u32);
    //         r_b = r_b ^ 5;
    //         r_b = r_b ^ r_c;
    //         r_a = r_a / 8;

    //         if out_index >= program.len() || r_b % 8 != program[out_index] {
    //             continue 'next;
    //         }
    //         out_index += 1;

    //         if r_a == 0 {
    //             break;
    //         }
    //     }


        // loop {
        //     if let Some(out) = cycle(&program, &mut r_ip, &mut r_a, &mut r_b, &mut r_c) {
        //         if out_index >= program.len() || out != program[out_index] {
        //             continue 'next;
        //         }
        //         out_index += 1;
        //     }
            
        //     if r_ip >= program.len() {
        //         break;
        //     }
        // }

    //     if out_index == program.len() {
    //         return a;
    //     }
    // }

    unreachable!()
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
