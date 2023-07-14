use std::collections::HashSet;
use itertools::{multizip, izip};

use crate::SudokuBoard;
use rand::{thread_rng, seq::SliceRandom};

fn init_solution(problem: &mut SudokuBoard) -> Vec<Vec<i32>>{
    let mut rng = thread_rng();
    let mut solution = Vec::new();
    for i in 0..problem.params.sudoku.size as usize{
        let row = problem.board[i].clone();
        let mut permutations: Vec<i32> = (1..=problem.params.sudoku.size).filter(|n| !row.contains(n)).collect();
        permutations.shuffle(&mut rng);
        let row_solution: Vec<i32> = row
            .iter()
            .enumerate()
            .map(|(j, &n)| if n == 0 { n | permutations.pop().unwrap() } else { n })
            .collect();
        solution.push(row_solution);
        
    }
    solution

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
    let mut current_solution = init_solution(&mut problem);
    let mut solutions = vec![problem.clone_params(current_solution)];
    let mut current_score = get_score(&solutions[0]);
    let mut current_temp = problem.params.simulated_annealing.initial_temp;
    while current_temp > problem.params.simulated_annealing.final_temp{
        
    }
}

