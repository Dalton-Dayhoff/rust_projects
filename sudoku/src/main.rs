use std::fs;

mod sudoku;
mod solve;

use solve::plan;
use sudoku::{AllParams, SudokuBoard};
fn main() {
    let filename = "parameters.toml";

    let contents = match fs::read_to_string(filename){
        Ok(c) => c,
        Err(e) => panic!("Could not find file {}", e)
    };

    let data: AllParams = match toml::from_str(&contents){
        Ok(d) => d,
        Err(e) => panic!("Could not parse file {}", e) 
    };
    let board  = SudokuBoard::new(data);
    plan(board);

}