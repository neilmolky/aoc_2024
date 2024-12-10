use std::{collections::VecDeque, env};

use itertools::Itertools;

use crate::error;

enum Memory {
    File { id: usize, _position: usize },
    Free { _position: usize },
}
impl Memory {
    fn parse_input(input: &str) -> VecDeque<Memory> {
        let (_, mem_chunks) = input.chars().enumerate().fold(
            (0, VecDeque::<Memory>::new()),
            |(position, mut acc), (i, c)| {
                let size: usize = c.to_digit(10).unwrap() as usize;
                if i % 2 == 0 {
                    (0..size).for_each(|p| {
                        acc.push_back(Memory::File {
                            id: i / 2,
                            _position: position + p,
                        })
                    });
                    (position + size, acc)
                } else {
                    (0..size).for_each(|p| {
                        acc.push_back(Memory::Free {
                            _position: position + p,
                        })
                    });
                    (position + size, acc)
                }
            },
        );
        mem_chunks
    }

    fn _position(&self) -> usize {
        match self {
            Memory::File {
                id: _,
                _position: position,
            } => *position,
            Memory::Free {
                _position: position,
            } => *position,
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum ContiguousMemory {
    File {
        id: usize,
        start: usize,
        size: usize,
    },
    Defrag {
        id: usize,
        start: usize,
        size: usize,
        _original_start: usize,
    },
    Free {
        start: usize,
        size: usize,
    },
}

impl ContiguousMemory {
    fn parse_input(input: &str) -> VecDeque<ContiguousMemory> {
        let (_, mem_chunks) = input.chars().enumerate().fold(
            (0, VecDeque::<ContiguousMemory>::new()),
            |(position, mut acc), (i, c)| {
                let size: usize = c.to_digit(10).unwrap() as usize;
                if i % 2 == 0 {
                    acc.push_back(ContiguousMemory::File {
                        id: i / 2,
                        start: position,
                        size,
                    });
                    (position + size, acc)
                } else {
                    if size > 0 {
                        acc.push_back(ContiguousMemory::Free {
                            start: position,
                            size,
                        });
                    }
                    (position + size, acc)
                }
            },
        );
        mem_chunks
    }
    fn start(&self) -> usize {
        match self {
            ContiguousMemory::File {
                id: _,
                start,
                size: _,
            } => *start,
            ContiguousMemory::Free { start, size: _ } => *start,
            ContiguousMemory::Defrag {
                id: _,
                start,
                size: _,
                _original_start: _,
            } => *start,
        }
    }
    fn end(&self) -> usize {
        self.start() + self.size()
    }
    fn size(&self) -> usize {
        match self {
            ContiguousMemory::File {
                id: _,
                start: _,
                size,
            } => *size,
            ContiguousMemory::Free { start: _, size } => *size,
            ContiguousMemory::Defrag {
                id: _,
                start: _,
                size,
                _original_start: _,
            } => *size,
        }
    }
    fn id(&self) -> usize {
        match self {
            ContiguousMemory::File {
                id,
                start: _,
                size: _,
            } => *id,
            ContiguousMemory::Free { start: _, size: _ } => panic!("Free does not have an id"),
            ContiguousMemory::Defrag {
                id,
                start: _,
                size: _,
                _original_start: _,
            } => *id,
        }
    }
}

pub fn part1(input: String) -> Result<String, error::Error> {
    let mut defrag: Vec<usize> = Vec::new();
    let mut mem_chunks = Memory::parse_input(&input);
    while let Some(x) = mem_chunks.pop_front() {
        match x {
            Memory::File { id, _position: _ } => defrag.push(id),
            _ => {
                while let Some(y) = mem_chunks.pop_back() {
                    match y {
                        Memory::File { id, _position: _ } => {
                            defrag.push(id);
                            break;
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    let result: usize = defrag.iter().enumerate().map(|(i, f)| i * f).sum();

    Ok(result.to_string())
}

fn prepend(mem_vec: &mut VecDeque<ContiguousMemory>, item: ContiguousMemory, dbg_str: &str) {
    match mem_vec.pop_front() {
        None => {
            mem_vec.push_front(item);
        }
        Some(first) => {
            if item.end() != first.start() {
                panic!(
                    "memory in {dbg_str} is not contiguous {:?} {:?}",
                    first, item
                )
            }
            mem_vec.push_front(first);
            mem_vec.push_front(item);
        }
    }
}

fn append(mem_vec: &mut VecDeque<ContiguousMemory>, item: ContiguousMemory, dbg_str: &str) {
    match mem_vec.pop_back() {
        None => {
            mem_vec.push_back(item);
        }
        Some(last) => {
            if last.end() != item.start() {
                panic!(
                    "memory in {dbg_str} is not contiguous {:?} {:?}",
                    last, item
                )
            }
            mem_vec.push_back(last);
            mem_vec.push_back(item);
        }
    }
}

fn scan(
    mem_chunks: &mut VecDeque<ContiguousMemory>,
    tail: &mut VecDeque<ContiguousMemory>,
    cache: &mut VecDeque<ContiguousMemory>,
) -> VecDeque<ContiguousMemory> {
    // defrag each result in mem_chunks
    while let Some(memory_for_defrag) = mem_chunks.pop_back() {
        match memory_for_defrag {
            ContiguousMemory::File {
                id,
                start: move_file_start,
                size: move_file_size,
            } => {
                let mut inserted = false;
                while let Some(insertion_position) = mem_chunks.pop_front() {
                    match insertion_position {
                        ContiguousMemory::Free {
                            start: free_mem_start,
                            size: free_mem_size,
                        } => {
                            if free_mem_start > move_file_start {
                                // file is not to the left and can't be inserted
                                prepend(
                                    tail,
                                    ContiguousMemory::Defrag {
                                        id,
                                        start: move_file_start,
                                        size: move_file_size,
                                        _original_start: move_file_start,
                                    },
                                    "tail",
                                );
                                inserted = true;
                                break;
                            } else if free_mem_size < move_file_size {
                                // file does not fit
                                append(cache, insertion_position, "cache");
                            } else if free_mem_size == move_file_size {
                                // file fits exactly
                                append(
                                    cache,
                                    ContiguousMemory::Defrag {
                                        id,
                                        start: free_mem_start,
                                        size: move_file_size,
                                        _original_start: move_file_start,
                                    },
                                    "cache",
                                );
                                prepend(
                                    tail,
                                    ContiguousMemory::Free {
                                        start: move_file_start,
                                        size: free_mem_size,
                                    },
                                    "tail",
                                );
                                inserted = true;
                                break;
                            } else {
                                // file fits and there is some remaining free space
                                let remaining_mem_size = free_mem_size - move_file_size;
                                let remaining_mem_start = free_mem_start + move_file_size;

                                append(
                                    cache,
                                    ContiguousMemory::Defrag {
                                        id,
                                        start: free_mem_start,
                                        size: move_file_size,
                                        _original_start: move_file_start,
                                    },
                                    "cache",
                                );
                                append(
                                    cache,
                                    ContiguousMemory::Free {
                                        start: remaining_mem_start,
                                        size: remaining_mem_size,
                                    },
                                    "cache",
                                );
                                prepend(
                                    tail,
                                    ContiguousMemory::Free {
                                        start: move_file_start,
                                        size: move_file_size,
                                    },
                                    "tail",
                                );
                                inserted = true;
                                break;
                            }
                        }
                        occupied => append(cache, occupied, "cache"),
                    }
                }
                if !inserted {
                    prepend(
                        tail,
                        ContiguousMemory::Defrag {
                            id,
                            start: move_file_start,
                            size: move_file_size,
                            _original_start: move_file_start,
                        },
                        "tail",
                    );
                }
                while let Some(value) = cache.pop_back() {
                    prepend(mem_chunks, value, "mem_chunks");
                }
            }
            x => {
                // no need to move free space or previously defragged files, add to tail
                prepend(tail, x, "tail");
            }
        }
    }
    // if defrag is empty, iteration complete return tail, else rebuild mem_chunks from defrag, this should be possible with a swap
    if mem_chunks.is_empty() & cache.is_empty() {
        tail.clone()
    } else {
        panic!("defrag still contains values at end of loop")
    }
}

pub fn part2(input: String) -> Result<String, error::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    let mut defrag: VecDeque<ContiguousMemory> = VecDeque::new();
    let mut tail: VecDeque<ContiguousMemory> = VecDeque::new();
    let mut mem_chunks = ContiguousMemory::parse_input(&input);

    let result_memory = scan(&mut mem_chunks, &mut tail, &mut defrag);

    let checksum: usize = result_memory
        .iter()
        .map(|x| match x {
            ContiguousMemory::Free { start: _, size: _ } => Vec::new(),
            x => (x.start()..x.end()).map(|i| x.id() * i).collect_vec(),
        })
        .flat_map(|x| x)
        .sum();
    Ok(checksum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2333133121414131402";
    #[test]
    fn test_part1() {
        let input = INPUT.to_string();
        let result = part1(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1928")
    }
    #[test]
    fn test_part2() {
        let input = INPUT.to_string();
        let result = part2(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "2858")
    }
}
