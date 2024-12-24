use std::collections::HashMap;

type Input<'a> = (
    Vec<((&'a str, &'a str, &'a str), &'a str)>,
    HashMap<&'a str, u32>,
);

aoc2024::main!("../../assets/day24.txt");

fn parse_input(input: &str) -> Input {
    let (initial, rules) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|l| {
            let (calc, res) = l.split_once(" -> ").unwrap();
            let mut calc = calc.split(" ");
            (
                (
                    calc.next().unwrap(),
                    calc.next().unwrap(),
                    calc.next().unwrap(),
                ),
                res,
            )
        })
        .collect::<Vec<_>>();

    let states = initial
        .lines()
        .map(|l| {
            let (n, v) = l.split_once(": ").unwrap();
            (n, v.parse::<u32>().unwrap())
        })
        .collect::<HashMap<_, _>>();

    (rules, states)
}

fn part1(input: &str) -> u64 {
    let (rules, mut states) = parse_input(input);

    simulate(&mut states, &rules)
}

fn simulate<'a>(
    states: &mut HashMap<&'a str, u32>,
    rules: &[((&str, &str, &str), &'a str)],
) -> u64 {
    let mut todo = rules.to_vec();
    
    loop {
        let mut i = todo.len() - 1;

        loop {
            let ((a, op, b), res) = &todo[i];

            if let Some((a, b)) = states.get(a).and_then(|a| states.get(b).map(|b| (a, b))) {
                let r = match *op {
                    "AND" => a & b,
                    "OR" => a | b,
                    "XOR" => a ^ b,
                    _ => unreachable!("Invalid operator"),
                };

                states.insert(res, r);
                todo.swap_remove(i);
            }

            if i == 0 {
                break;
            }
            i -= 1;
        }

        if todo.is_empty() {
            break;
        }
    }

    states
        .iter()
        .filter(|(n, _)| n.starts_with("z"))
        .fold(0, |acc, (n, v)| {
            let decimal = n[1..].parse::<u64>().unwrap();
            acc + ((*v as u64) << decimal)
        })
}

fn part2(input: &str) -> u32 {
    let (rules, states) = parse_input(input);

    // obvious wrong rules
    for ((_, op, _), res) in &rules {
        if res.starts_with("z") && *res != "z45" && *op != "XOR" {
            println!("Wrong rule: {:?}", res);
        }
    }

    // fkb,nnr,rdn,rqf,rrn,z16,z31,z37
    let rules = rules
        .iter()
        .map(|(op, res)| match *res {
            "z16" => (*op, "fkb"),
            "fkb" => (*op, "z16"),
            "z31" => (*op, "rdn"),
            "rdn" => (*op, "z31"),
            "z37" => (*op, "rrn"),
            "rrn" => (*op, "z37"),
            "rqf" => (*op, "nnr"),
            "nnr" => (*op, "rqf"),
            r => (*op, r),
        })
        .collect::<Vec<_>>();

    // test each bit seperately
    for i in 0..45 {
        let mut states = states.clone();

        // set every value to 0 except for current bits
        let x = format!("x{:02}", i);
        let y = format!("y{:02}", i);
        for (n, v) in &mut states {
            if *n == x || *n == y {
                *v = 1;
            } else {
                *v = 0;
            }
        }

        let output = simulate(&mut states, &rules);

        println!(
            "Bit {:>2} -> {:>5}: {:b}",
            i,
            output == 1 << (i + 1),
            output
        );
    }

    0
}

aoc2024::test!(
    "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
",
    2024,
    0
);
