use text_io::read;
use std::{collections::HashMap, vec};
use rand;

const MAX_LINES:i32 = 5;
const MAX_BET: f64 = 50.0;
const MIN_BET: f64 = 0.5;
const VISIBLE_SPOTS: i32 = 5;
const NUMBER_OF_WHEELS: i32 = 5;


fn deposit() -> f64{
    let indicator = true;
    let mut amount: f64 = 0.0;
    while indicator == true{
        print!("How much would you like to deposit? $");
        let input: String = read!("{}\n");
        let test = input.parse::<f64>();
            match test {
                Ok(ok) => amount = ok,
                Err(_e) => {println!("Invalid amount, please enter a number"); 
                                            continue},
            }
        if amount > 0.0{
            break;
        }
        println!("Invalid amount, please enter a positive number.")
    }
    bankers_round(amount)
}

fn get_number_of_lines() -> i32{
    let indicator = true;
    let mut lines: i32 = 0;
    while indicator == true{
        print!("Enter the number of lines to play (1 - {}). ", MAX_LINES);
        let input: String = read!("{}\n");
        let test = input.parse::<i32>();
            match test {
                Ok(ok) => lines = ok,
                Err(_e) => {println!("Invalid input, please enter a number"); 
                                            continue},
            }
        if lines > 0 && lines <= MAX_LINES{
            break;
        }
        println!("Invalid amount, please enter a valid number of lines")
    }
    lines
}

fn get_bet() -> f64{
    let indicator = true;
    let mut bet: f64 = 0.0;
    while indicator == true{
        print!("How much would you like to bet per line (${} - ${}) $", MIN_BET, MAX_BET);
        let input: String = read!("{}\n");
        let test = input.parse::<f64>();
            match test {
                Ok(ok) => bet = ok,
                Err(_e) => {println!("Invalid amount, please enter a number"); 
                                            continue},
            }
        if bet >= MIN_BET && bet <= MAX_BET{
            break;
        }
        println!("Invalid amount, please enter a valid bet.")
    }
    bankers_round(bet)
}


fn bankers_round(value: f64) -> f64{
    (value*100.0).round()/100.0
}

fn get_slot_machine_spin(rows: i32, cols: i32, symbols: &HashMap<char, i32>) -> Vec<Vec<char>>{
    let mut all_symbols: Vec<char> = Vec::new();
    for (symbol, symbol_count) in &*symbols{
        for _ in 0..*symbol_count{
            all_symbols.push(*symbol);
        }
    }
    let mut columns: Vec<Vec<char>> = vec![];
    for _ in 0..cols{
        let mut column: Vec<char> = Vec::new();
        let mut current_symbols = all_symbols.clone();
        for _ in 0..rows{
            let index = (rand::random::<f32>() * current_symbols.len() as f32).floor() as usize;
            let value = current_symbols.remove(index);
            column.push(value);
        }
        columns.push(column)
    }
    columns
}

fn print_slot_machine(columns: &Vec<Vec<char>>){
    for row in 0..columns[0].len(){
        for (i, col) in columns.iter().enumerate(){
            if i != columns.len() - 1{
                print!("{} | ", col[row])
            }
            else{
                println!("{}", col[row])
            }
        }
    }
}

fn check_winnings(columns: &Vec<Vec<char>>, lines: i32, bet: f64, values: &HashMap<char, i32>) -> (f64, Vec<i32>){
    let mut winnings = 0.0;
    let mut winning_lines: Vec<i32> = Vec::new();
    let mut symbol: char;
    let mut indicator: bool;
    for line in 0..lines{
        symbol = columns[0][line as usize];
        indicator = true;
        for column in columns.iter(){
            let symbol_to_check = column[line as usize];
            if symbol != symbol_to_check{
                indicator = false;
                break;
            }
        }
        if indicator == true{
            winnings += values[&symbol] as f64 * bet;
            winning_lines.push(line + 1);
        }
    }
    (winnings, winning_lines)
}

fn slot_driver(symbols: &HashMap<char, i32>, values: &HashMap<char, i32>, account:&mut f64){
    let mut number_of_lines = get_number_of_lines();
    let mut bet = get_bet();
    let mut total_bet = bet*number_of_lines as f64;
    loop{
        if total_bet > *account{
            println!("You do not have enough money in your account for the bet you have specified.");
            for i in 0..NUMBER_OF_WHEELS{
                println!("Your max bet for {} line(s) is ${}", i + 1, bankers_round(*account/(i as f64 + 1.0)));
            }
            number_of_lines = get_number_of_lines();
            bet = get_bet();
            total_bet = bet*number_of_lines as f64;
            continue;
        }
        *account -= total_bet;
        break;
    }
    println!("You are betting ${} per line on {} line(s) for a total of ${}", bet, number_of_lines, total_bet);
    let slots = get_slot_machine_spin(VISIBLE_SPOTS, NUMBER_OF_WHEELS, &symbols);
    print_slot_machine(&slots);
    let (winnings, winning_lines) = check_winnings(&slots, number_of_lines, bet, values);
    *account += winnings;
    println!("You won ${}, your total account balance is now ${}.", winnings, account);
    if winning_lines.len() != 0{
        print!("You won on lines ");
        for line in winning_lines.iter(){
            print!("{line}");
        }
        println!();
    }
}
fn main() {
    let symbols: HashMap<char, i32> = HashMap::from([
        ('A', VISIBLE_SPOTS),
        ('B', VISIBLE_SPOTS*2),
        ('C', VISIBLE_SPOTS*3),
        ('D', VISIBLE_SPOTS*4)
    ]);
    let values: HashMap<char, i32> = HashMap::from([
        ('A', 5),
        ('B', 4),
        ('C', 3),
        ('D', 2)
    ]);
    let mut account = deposit();
    loop {
        slot_driver(&symbols, &values, &mut account);
        print!("Press enter to play again (q to quit). ");
        let input: String = read!("{}\n");
        if input == "q"{
            break;
        }
        print!("Press enter to continue or a to add more to your account. ");
        let input_2: String = read!("{}\n");
        if input_2 == "a"{
            account += deposit();
        }
        continue;
        
    }
}
