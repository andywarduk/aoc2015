use memmap::Mmap;
use std::{fs::File, io::{BufRead, BufReader}};

type Board = Vec<Vec<char>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let board = load_input("input18.txt")?;

    part1(&board);

    part2(&board);

    Ok(())
}

fn part1(init_board: &Board) {
    let mut board = init_board.clone();

    for _ in 0..100 {
        board = step(board)
    }

    let lit: usize = board.iter().map(|row| {
        row.iter().filter(|c| **c == '#').count()
    }).sum();

    println!("{} lights lit (part 1)", lit);
}

fn part2(init_board: &Board) {
    let mut board = init_board.clone();

    fix_corners(&mut board);

    for _ in 0..100 {
        board = step(board);

        fix_corners(&mut board);
    }

    let lit: usize = board.iter().map(|row| {
        row.iter().filter(|c| **c == '#').count()
    }).sum();

    println!("{} lights lit (part 2)", lit);
}

fn fix_corners(board: &mut Board) {
    let max_x = board[0].len() - 1;
    let max_y = board.len() - 1;

    board[0][0] = '#';
    board[max_y][0] = '#';
    board[0][max_x] = '#';
    board[max_y][max_x] = '#';
}

fn step(board: Board) -> Board {
    let mut new_board = Vec::with_capacity(board.len());

    for y in 0..board.len() {
        let row = &board[y];
        let mut new_row = Vec::with_capacity(row.len());

        for x in 0..row.len() {
            let new_state = calc_state(&board, x, y);
            new_row.push(new_state);
        }

        new_board.push(new_row);
    }

    new_board
}

fn calc_state(board: &Board, x: usize, y: usize) -> char {
    let max_x = board[0].len() - 1;
    let max_y = board.len() - 1;

    let mut count = 0;

    let mut look = |x: usize, y: usize| {
        if board[y][x] == '#' {
            count += 1;
        }
    };

    if y > 0 {
        if x > 0 {
            look(x - 1, y - 1);
        }
        look(x, y - 1);
        if x < max_x {
            look(x + 1, y - 1);
        }
    }
    if x > 0 {
        look(x - 1, y);
    }
    if x < max_x {
        look(x + 1, y);
    }
    if y < max_y {
        if x > 0 {
            look(x - 1, y + 1);
        }
        look(x, y + 1);
        if x < max_x {
            look(x + 1, y + 1);
        }
    }

    if board[y][x] == '#' {
        match count {
            2 | 3 => '#',
            _ => '.'
        }
    } else {
        match count {
            3 => '#',
            _ => '.'
        }
    }
}

fn load_input(file: &str) -> Result<Board, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Create buf reader for mmapped file
    let buf_reader = BufReader::new(mmap.as_ref());

    let mut lines = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            lines.push(line.chars().collect());
        }
    }

    Ok(lines)
}
