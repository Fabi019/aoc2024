
#[derive(Debug, Clone)]
struct Block(Option<usize>, u32);   // id, size

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

    loop {
        let cblocks = blocks.clone();

        let first_free = blocks.iter().position(|b| b.0.is_none() && b.1 != 0).unwrap_or(usize::MAX);
        if first_free == usize::MAX {
            break;
        }

        let last_file = blocks.len() - 1 - blocks.iter().rev().position(|b| b.0.is_some()).unwrap();

        // fully compacted
        if first_free > last_file {
            break;
        }

        let free = &cblocks[first_free];
        let file = &cblocks[last_file];

        if free.1 > file.1 {
            // insert fully
            blocks.remove(last_file);
            blocks.insert(first_free, file.clone());
            blocks[first_free + 1].1 -= file.1; // reduce free space size
        } else {
            // insert partially, fully replace free
            blocks[first_free] = Block(file.0, free.1);
            if file.1 == free.1 {
                blocks.remove(last_file);   // completely remove file
            } else {
                blocks[last_file].1 -= free.1; // reduce remaining file size
            }
        }
    }

    let mut index = 0;
    blocks.into_iter().filter(|b| b.0.is_some()).fold(0, |mut acc,  b| {
        for i in 0..b.1 {
            acc += b.0.unwrap() as u64 * (index + i as u64);
        }
        index += b.1 as u64;
        acc
    })
}

fn part2(input: &str) -> u64 {
    let mut blocks = vec![];
    let mut last_id = 0;

    for (i, c) in input.lines().next().unwrap().char_indices() {
        let size = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            blocks.push(Block(Some(i / 2), size));
            last_id = i / 2;
        } else {
            blocks.push(Block(None, size));
        }
    }
    
    let mut start_id = last_id;

    loop {
        let cblocks = blocks.clone();

        if start_id == 0 {
            break; // done
        }

        let current_file = blocks.len() - 1 - blocks.iter().rev().position(|b| b.0.is_some() && b.0.unwrap() == start_id).unwrap();
        let block = &cblocks[current_file];
        
        for (i, free) in cblocks.iter().enumerate().filter(|(_, b)| b.0.is_none() && b.1 != 0) {
            if i > current_file {
                break;  // free space is after file
            }

            if block.1 > free.1 {
                continue; // free block not large enough
            }

            if block.1 == free.1 {
                blocks.swap(i, current_file);
            } else {
                blocks[current_file].0 = None;
                blocks.insert(i, block.clone());
                blocks[i+1].1 -= block.1;
            }

            break;
        }
        
        start_id -= 1;
    }

    let mut index = 0;
    blocks.into_iter().fold(0, |mut acc,  b| {
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
", 1928, 2858
);
