use serde_derive::Deserialize;
use std::{fs, vec};
use rand::{seq::SliceRandom, Rng};
use toml;

#[derive(Deserialize, Debug, Clone, Copy)]
enum Difficulty{
    Easy = 40,
    Medium = 50,
    Hard = 60,
    Extreme = 70
}


#[derive(Deserialize, Clone)]
pub struct SimulatedAnnealingParams{
    pub initial_temp: f64,
    pub cooling_rate: f64,
    pub final_temp: f64,
}

#[derive(Deserialize, Clone)]
pub struct SudokuParams {
    pub size: i32,
    difficulty: Difficulty
}


#[derive(Deserialize, Clone)]
pub struct AllParams {
    pub sudoku: SudokuParams,
    pub simulated_annealing: SimulatedAnnealingParams
}

pub struct SudokuBoard {
    pub board: Vec<Vec<i32>>,
    pub params: AllParams
}

impl SudokuBoard{
    pub fn new(params: AllParams) -> Self{
        let size = params.sudoku.size.clone() as usize;
        let mut board = vec![vec![0; size]; size];
        let mut valid_board = SudokuBoard::random_puzzle_from_empty(&mut board, size);
        while !valid_board{
            board = vec![vec![0; size]; size];
            valid_board = SudokuBoard::random_puzzle_from_empty(&mut board, size);
        }
        let mut sudoku =  Self { board: board, params: params };
        sudoku.puzzilify();
        sudoku
    } 
    pub fn clone_params(&self, new_board: Vec<Vec<i32>>) -> SudokuBoard {
        SudokuBoard { board: new_board, params: self.params.clone() }
    }

    fn random_puzzle_from_empty(board: &mut Vec<Vec<i32>>, size: usize) -> bool{
        for row in 0..size as usize{
            for col in 0..size as usize{
                if board[row][col] == 0 {
                    let mut rng= rand::thread_rng();
                    let mut possible_numbers: Vec<i32> = (1..=size as i32).collect();
                    possible_numbers.shuffle(&mut rng);

                    for num in possible_numbers{
                        if SudokuBoard::is_safe(&board, size as i32, row, col, num){
                            board[row][col] = num;

                            if Self::random_puzzle_from_empty(board, size){
                                return true;
                            }
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    pub fn removable(&self, i: usize, j: usize)-> bool{
        if self.board[i][j] == 0 {
            return true;
        }
        false
    }

    fn is_safe(board: &Vec<Vec<i32>>, size: i32, row: usize, col: usize, num: i32) -> bool{
        let box_size = (size as f64).sqrt() as usize;
        !SudokuBoard::used_in_row(board, row, num) 
            && !SudokuBoard::used_in_col(board, col, num) 
            && !SudokuBoard::used_in_square(
                board, 
                col - col % box_size, 
                row - row % box_size, 
                num, 
                box_size)
    }

    fn used_in_row(board: &Vec<Vec<i32>>, row: usize, num: i32) -> bool{
        board[row].contains(&num)
    }
    
    fn used_in_col(board: &Vec<Vec<i32>>, col: usize, num: i32) -> bool {
        board.iter().any(|row| row[col] == num)
    }
    
    fn used_in_square(board: &Vec<Vec<i32>>, col_start: usize, row_start: usize, num: i32, box_size: usize) -> bool {
        for i in 0..box_size{
            for j in 0..box_size{
                if board[row_start + i][col_start + j] == num{
                    return true;
                }
            }
        }
        return false;
    }

    fn puzzilify(&mut self){
        for _  in 0..self.params.sudoku.difficulty as usize{
            let mut found_removable_value = false;
            let mut rng = rand::thread_rng();
            while !found_removable_value{
                let row = rng.gen_range(0..9);
                let col = rng.gen_range(0..9);
                if self.board[row][col] == 0 {
                    continue;
                }
                else{
                    self.board[row][col] = 0;
                    found_removable_value = true;
                }
            }
        }
    }
}

impl std::fmt::Display for SudokuBoard{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let block_size = (self.params.sudoku.size.clone() as f64).sqrt() as usize + 1;
        let size_p2 = self.params.sudoku.size.clone() as usize + block_size - 1;
        for i in 0..=size_p2{
            for j in 0..=size_p2{
                match (i, j) {
                    (row, _col) if row == 0 || row % block_size == 0 || row == size_p2=> write!(f, "- ")?,
                    (_row, col) if col == size_p2 || col == 0 || col % block_size == 0 => write!(f, "| ")?,
                    _ => {
                        let mut actual_row = 0;
                        let mut actual_col = 0;
                        find_actual_spot(&mut actual_row, &block_size, &i);
                        find_actual_spot(&mut actual_col, &block_size, &j);
                        if self.board[actual_row][actual_col] == 0{
                            write!(f, "  ")?;
                        }
                        else{
                            write!(f, "{} ", self.board[actual_row][actual_col])?;
                        }
                    }
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn find_actual_spot(actual_spot: &mut usize, block: &usize, index: &usize) {
    match index {
        i if *i < block + 1 => *actual_spot = *i - 1,
        i if *i < (2 * block + 1) => *actual_spot = *i - 2,
        _ => *actual_spot = index - 3,
    }
}