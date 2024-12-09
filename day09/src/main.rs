use std::{collections::VecDeque, io::Read as _};

#[derive(Clone, Copy, Debug)]
enum Block {
    File(u32),
    Free,
}

fn part1(input: &[u8]) {
    let mut is_file = true;
    let mut file_id = 0;
    let mut block = 0;

    let mut blocks = Vec::new();
    let mut file_blocks = Vec::new();
    let mut free_blocks = VecDeque::new();

    for byte in input {
        if !byte.is_ascii_digit() {
            break;
        }
        let len = byte - 0x30;
        for _ in 0..len {
            if is_file {
                blocks.push(Block::File(file_id));
                file_blocks.push(block);
            } else {
                blocks.push(Block::Free);
                free_blocks.push_back(block);
            }
            block += 1;
        }

        if is_file {
            file_id += 1;
        }
        is_file = !is_file;
    }

    let first_free_index_when_done = file_blocks.len();
    while free_blocks
        .iter()
        .any(|index| *index < first_free_index_when_done)
    {
        let file_block_index = file_blocks.pop().unwrap();
        let free_block_index = free_blocks.pop_front().unwrap();
        blocks[free_block_index] = blocks[file_block_index];
        blocks[file_block_index] = Block::Free;
        free_blocks.push_back(file_block_index);
    }

    let mut checksum = 0;
    for (i, block) in blocks.into_iter().enumerate() {
        match block {
            Block::File(file_id) => {
                checksum += u64::try_from(i).unwrap() * u64::from(file_id);
            }
            Block::Free => break,
        }
    }
    println!("{checksum}");
}

#[derive(Clone, Copy, Debug)]
struct File {
    id: u32,
    index: usize,
    length: u32,
}

#[derive(Clone, Copy, Debug)]
struct Free {
    index: usize,
    length: u32,
}

fn part2(input: &[u8]) {
    let mut is_file = true;
    let mut file_id = 0;
    let mut index = 0;

    let mut files = VecDeque::new();
    let mut frees = Vec::new();

    for byte in input {
        if !byte.is_ascii_digit() {
            break;
        }
        let length = byte - 0x30;
        if is_file {
            files.push_back(File {
                id: file_id,
                index,
                length: length.into(),
            });
        } else {
            frees.push(Free {
                index,
                length: length.into(),
            });
        }

        if is_file {
            file_id += 1;
        }
        index += usize::from(length);
        is_file = !is_file;
    }

    loop {
        if files.back().unwrap().index == 0 {
            break;
        }

        let file = files.pop_back().unwrap();
        let Some(free_pos) = frees.iter().position(|free| free.length >= file.length) else {
            files.push_front(file);
            continue;
        };
        let free = frees[free_pos];
        if free.index > file.index {
            files.push_front(file);
            continue;
        }
        files.push_front(File {
            id: file.id,
            index: free.index,
            length: file.length,
        });
        frees[free_pos].index += usize::try_from(file.length).unwrap();
        frees[free_pos].length -= file.length;

        if let Some(free) = frees
            .iter_mut()
            .find(|free| free.index == file.index + usize::try_from(file.length).unwrap())
        {
            free.length += file.length;
        } else {
            frees.push(Free {
                index: file.index,
                length: file.length,
            });
        }
    }

    let mut checksum = 0;
    for file in files {
        for i in file.index..file.index + usize::try_from(file.length).unwrap() {
            checksum += u64::try_from(i).unwrap() * u64::from(file.id);
        }
    }

    println!("{checksum}");
}

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
    part1(&input);
    part2(&input);
}
