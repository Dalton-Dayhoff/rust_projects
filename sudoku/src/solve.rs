use std::collections::HashSet;

use crate::SudokuBoard;
use rand::{thread_rng, Rng};

fn init_solution(problem: &SudokuBoard) -> Vec<Vec<i32>> {
    let mut rng = thread_rng();
    let mut solution = vec![vec![0; problem.params.sudoku.size as usize]; problem.params.sudoku.size as usize];
    let block_size = (problem.params.sudoku.size as f64).sqrt() as usize;
    for block_row in (0..problem.params.sudoku.size as usize).step_by(block_size){
        for block_col in (0..problem.params.sudoku.size as usize).step_by(block_size) {
            let mut missing_numbers = get_block_numbers(problem, block_col, block_row, block_size);
            for i in block_row..(block_row + block_size){
                for j in block_col..(block_col + block_size) {
                    if problem.removable(i, j){
                        let index = rng.gen_range(0..missing_numbers.len());
                        solution[i][j] = missing_numbers.remove(index)
                    }
                    else {
                        solution[i][j] = problem.board[i][j]
                    }
                }
            }
        }
    }
    solution
}

fn get_missing_numbers(block: &Vec<i32>, size: i32) -> Vec<i32>{
    let mut numbers: Vec<i32> = (0..=size).into_iter().collect();
    numbers.retain(|number| !block.contains(number));
    numbers
}

fn get_block_numbers(problem: &SudokuBoard, col_start: usize, row_start: usize, block_size: usize) -> Vec<i32>{
    let mut numbers = HashSet::new();
    for row in row_start..(row_start + block_size) {
        for col in col_start..(col_start + block_size) {
            numbers.insert(problem.board[row][col]);
        }
    }
    get_missing_numbers(&numbers.into_iter().collect(), problem.params.sudoku.size)
}

fn get_score(problem: &SudokuBoard) -> i32{
    let inverted: Vec<Vec<i32>> = (0..problem.params.sudoku.size as usize).map(|col| {
        (0..problem.params.sudoku.size as usize)
            .map(|row| problem.board[row][col])
            .collect()
    }).collect();
    let mut score = 0;
    for i in 0..problem.params.sudoku.size as usize{
        let row = problem.board[i].clone();
        score += 9 -unique(&row);
        let col = inverted[i].clone();
        score += 9 - unique(&col);
    }
    score as i32
}

fn unique(row: &Vec<i32>) -> usize{
    let mut unique_items: HashSet<i32> = HashSet::new();
    unique_items.extend(row.iter().copied());
    let result: Vec<i32> = unique_items.into_iter().collect();
    result.len()
}

pub fn plan(mut problem: SudokuBoard){
    let mut current_solution = init_solution(&problem);
    let mut solutions = vec![problem.clone_params(current_solution)];
    let mut current_score = get_score(&solutions[0]);
    let mut current_temp = problem.params.simulated_annealing.initial_temp;
    println!("{}", solutions[0]);
    println!("{}", current_score);
    // while current_temp > problem.params.simulated_annealing.final_temp{
    //     for i in 0..problem.params.simulated_annealing.max_pass_iter as usize{

    //     }
    // }
}

