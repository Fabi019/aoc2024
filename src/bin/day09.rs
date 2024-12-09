#[derive(Debug, Clone)]
struct Block(Option<usize>, u32); // id, size

aoc2024::main!("../../assets/day09.txt");

fn part1(input: &str) -> u64 {
    let mut blocks = vec![];

    for (i, c) in input.lines().next().unwrap().char_indices() {
        let size = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            blocks.push(Block(Some(i / 2), size));
        } else {
            blocks.push(Block(None, size));
        }
    }

    let mut start = 0;

    loop {
        let mut first_free = usize::MAX;
        for (i, b) in blocks.iter().enumerate().skip(start) {
            if b.0.is_none() && b.1 != 0 {
                first_free = i;
                start = i;
                break;
            }
        }

        if first_free == usize::MAX {
            break; // no free space left
        }

        let last_file = blocks.len() - 1 - blocks.iter().rev().position(|b| b.0.is_some()).unwrap();

        if first_free > last_file {
            break; // fully compacted
        }

        let free = blocks[first_free].clone();
        let file = blocks[last_file].clone();

        if free.1 > file.1 {
            // insert fully, split free
            blocks.remove(last_file);
            blocks[first_free].1 -= file.1; // reduce free space size
            blocks.insert(first_free, file);
        } else {
            // insert partially, replace free
            blocks[first_free].0 = file.0;
            if file.1 == free.1 {
                blocks.remove(last_file); // completely remove file
            } else {
                blocks[last_file].1 -= free.1; // reduce remaining file size
            }
        }
    }

    let mut index = 0;
    blocks.into_iter().fold(0, |mut acc, b| {
        if let Some(id) = b.0 {
            for i in 0..b.1 {
                acc += id as u64 * (index + i as u64);
            }
        }
        index += b.1 as u64;
        acc
    })
}

fn part2(input: &str) -> u64 {
    let mut blocks = vec![];
    let mut current_id = 0;

    for (i, c) in input.lines().next().unwrap().char_indices() {
        let size = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            blocks.push(Block(Some(i / 2), size));
            current_id = i / 2;
        } else {
            blocks.push(Block(None, size));
        }
    }

    let mut end = blocks.len();

    loop {
        let mut current_file = 0;
        for i in (0..end).rev() {
            if blocks[i].0 == Some(current_id) {
                current_file = i;
                end = i;
                break;
            }
        }

        let block = blocks[current_file].clone();

        for i in 0..current_file {
            let free = &blocks[i];
            if free.0.is_some() {
                continue; // no free space
            }

            if block.1 > free.1 {
                continue; // free block not large enough
            }

            if block.1 == free.1 {
                blocks.swap(i, current_file);
            } else {
                blocks[current_file].0 = None;
                blocks[i].1 -= block.1;
                blocks.insert(i, block);
            }

            break;
        }

        current_id -= 1;
        if current_id == 0 {
            break; // done
        }
    }

    let mut index = 0;
    blocks.into_iter().fold(0, |mut acc, b| {
        if let Some(id) = b.0 {
            for i in 0..b.1 {
                acc += id as u64 * (index + i as u64);
            }
        }
        index += b.1 as u64;
        acc
    })
}

aoc2024::test!(
    "\
2333133121414131402
",
    1928,
    2858
);
