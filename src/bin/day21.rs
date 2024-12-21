use std::collections::HashMap;

const KEYPAD: [(char, (usize, usize)); 11] = [
    ('0', (1, 3)),
    ('1', (0, 2)),
    ('2', (1, 2)),
    ('3', (2, 2)),
    ('4', (0, 1)),
    ('5', (1, 1)),
    ('6', (2, 1)),
    ('7', (0, 0)),
    ('8', (1, 0)),
    ('9', (2, 0)),
    ('A', (2, 3)),
];
const DPAD: [(char, (usize, usize)); 5] = [
    ('<', (0, 1)),
    ('>', (2, 1)),
    ('^', (1, 0)),
    ('v', (1, 1)),
    ('A', (2, 0)),
];
const DIRECTIONS: [(char, (isize, isize)); 4] =
    [('<', (-1, 0)), ('^', (0, -1)), ('v', (0, 1)), ('>', (1, 0))]; // prefer left before up/down before right

aoc2024::main!("../../assets/day21.txt");

fn part1(input: &str) -> usize {
    let keypad = HashMap::from(KEYPAD);
    let dpad = HashMap::from(DPAD);

    let mut complexity = 0;
    let mut cache = HashMap::new();

    for seq in input.lines().map(|l| "A".to_owned() + l) {
        // convert digits to directions
        let keys = seq
            .as_bytes()
            .windows(2)
            .flat_map(|ct| map_char(&keypad, ct[0] as char, ct[1] as char, (0, 3)));

        let mut sum = 0;
        let mut last = 'A';
        for c in keys {
            sum += count_recursive(&dpad, &mut cache, last, c, (0, 0), 2);
            last = c;
        }

        complexity += seq[1..seq.len() - 1].parse::<usize>().unwrap() * sum;
    }

    complexity
}

fn count_recursive(
    charmap: &HashMap<char, (usize, usize)>,
    cache: &mut HashMap<(char, char, usize), usize>,
    c: char,
    t: char,
    hole: (usize, usize),
    robot: usize,
) -> usize {
    if let Some(s) = cache.get(&(c, t, robot)) {
        return *s;
    }

    let s = map_char(charmap, c, t, hole);

    if robot == 1 {
        cache.insert((c, t, robot), s.len());
        return s.len();
    }

    let mut sum = 0;
    let mut last = 'A';
    for c in s {
        sum += count_recursive(charmap, cache, last, c, hole, robot - 1);
        last = c;
    }

    cache.insert((c, t, robot), sum);
    sum
}

fn map_char(
    charmap: &HashMap<char, (usize, usize)>,
    c: char,
    t: char,
    hole: (usize, usize),
) -> Vec<char> {
    let mut sequence = Vec::new();
    let (mut cx, mut cy) = charmap[&c];
    let (tx, ty) = charmap[&t];
    let mut d = 0;

    while cx != tx || cy != ty {
        let (cd, (dx, dy)) = DIRECTIONS[d % 4];
        d += 1;

        // check if relevant direction
        if cx > tx && dx > 0
            || cx < tx && dx < 0
            || cx == tx && dx != 0
            || cy > ty && dy > 0
            || cy < ty && dy < 0
            || cy == ty && dy != 0
        {
            continue;
        }

        let ax = dx * cx.abs_diff(tx) as isize;
        let ay = dy * cy.abs_diff(ty) as isize;

        let nx = cx.wrapping_add_signed(ax);
        let ny = cy.wrapping_add_signed(ay);

        if (nx, ny) == hole {
            continue;
        }

        for _ in 0..(ax.abs() + ay.abs()) {
            sequence.push(cd);
        }

        cx = nx;
        cy = ny;
    }

    sequence.push('A');
    sequence
}

fn part2(input: &str) -> usize {
    let keypad = HashMap::from(KEYPAD);
    let dpad = HashMap::from(DPAD);

    let mut complexity = 0;
    let mut cache = HashMap::new();

    for seq in input.lines().map(|l| "A".to_owned() + l) {
        // convert digits to directions
        let keys = seq
            .as_bytes()
            .windows(2)
            .flat_map(|ct| map_char(&keypad, ct[0] as char, ct[1] as char, (0, 3)));

        let mut sum = 0;
        let mut last = 'A';
        for c in keys {
            sum += count_recursive(&dpad, &mut cache, last, c, (0, 0), 25);
            last = c;
        }

        complexity += seq[1..seq.len() - 1].parse::<usize>().unwrap() * sum;
    }

    complexity
}

aoc2024::test!(
    "\
029A
980A
179A
456A
379A
",
    126384,
    154115708116294
);
