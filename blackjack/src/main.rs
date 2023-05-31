mod deck_of_cards;
use std::fs;
use text_io::read;
use rand::{self, Rng};
use deck_of_cards::{Hand, Deck, Card};

#[derive(serde_derive::Deserialize)]
struct Variables {
    number_of_players: i32,
    number_of_shuffles: i32,
}

fn main() {
    let file_name = "./src/variables.toml";
    let contents: String = match fs::read_to_string(file_name){
        Ok(file) => file,
        Err(e) => panic!("Could not open file {}", e),
    };
    let map: Variables = match toml::from_str(&contents) {
        Ok(data) => data,
        Err(e) => panic!("{}", e), 
    };
    let mut card_deck = Deck::set_deck();
    let mut players: Vec<Hand> = vec![Hand::new_hand(); map.number_of_players.try_into().unwrap()];
    let mut dealer = Hand::new_hand();
    loop {
        for _ in [0..map.number_of_shuffles]{ // Shuffle Deck
            card_deck.shuffle();
        }
        for _ in 0..2{ // Deal Cards
            for i in 0..map.number_of_players + 1 { // Each full run through of this loop gives everyone a card
                if i == map.number_of_players {
                    dealer.add_card(card_deck.deal_card());
                }
                else{
                    players[i as usize].add_card(card_deck.deal_card());
                }
            }
        }
        // Print Initial Hands
        println!("Player Cards");
        for (i, hand) in players.iter().enumerate() {
            print!("Player {}'s cards are: ", i + 1);
            hand.print_hand();
            println!()
        }
        dealer.print_dealer_hand();

        // Calculate Initial Hand Totals
        let mut sums = Vec::new();
        for hand in players.iter(){
            sums.push(hand.calculate_sum());
        }
        let mut dealer_score = dealer.calculate_sum();
        let mut still_playing = vec![true; map.number_of_players as usize];
        // Check for blackjack
        check_for_blackjack(&sums, &mut still_playing);
        if dealer_score == 21{
            println!("Dealer wins with a blackjack");
            break;
        }
        // Player's turns if they don't have blackjack
        for (i, hand) in players.iter_mut().enumerate(){
            if still_playing[i] == false{
                continue;
            }
            let mut turn:String;
            println!("Player {}'s Turn!", i + 1);
            loop {
                println!("Your total: {}", sums[i]);
                print!("What would you like to do (hit or stay)? ");
                let input: String = read!("{}\n");
                let test = input.parse::<String>();
                    match test {
                        Ok(answer) => turn = answer,
                        Err(_) => {println!("Please enter a string"); continue;}
                    }
                let generalized_turn = turn.to_lowercase();
                if generalized_turn == "hit"{
                    hand.add_card(card_deck.deal_card());
                    print!("Your new hand is ");
                    hand.print_hand();
                    println!();
                    sums[i] = hand.calculate_sum();
                    if sums[i] > 21{
                        println!("You busted!");
                        still_playing[i] = false;
                        break;
                    }
                    else if sums[i] == 21 {
                        println!("21! You win.");
                        still_playing[i] = false;
                        break;
                    }
                }
                else if generalized_turn == "stay" {
                    break;
                }
                else {
                    println!("Please enter a valid command");
                    continue;
                }
            }
        }
        // Dealer's Turn
        let mut rng = rand::thread_rng();
        loop {
            if dealer_score > 17{
                break;
            }
            else if dealer_score == 17 && rng.gen_range(0..100) >= 99{
                dealer_hit(&mut dealer_score, &mut dealer, &mut card_deck);
                continue;
            }
            else if dealer_score == 16 && rng.gen_range(0..100) >= 80{
                dealer_hit(&mut dealer_score, &mut dealer, &mut card_deck);
                continue;
            }
            else if dealer_score == 15 && rng.gen_range(0..100) >= 40{
                dealer_hit(&mut dealer_score, &mut dealer, &mut card_deck);
                continue;
            }
            dealer_hit(&mut dealer_score, &mut dealer, &mut card_deck);
        }
        

    }

}

fn dealer_hit(mut dealer_score: &mut i32, mut dealer_hand: &mut Hand, mut card_deck: &mut Deck) {
    dealer_hand.add_card(card_deck.deal_card());
    *dealer_score = dealer_hand.calculate_sum();
}

fn check_for_blackjack(sums:&Vec<i32>, still_playing: &mut Vec<bool>) {
    if sums.contains(&21){
        for (i, sum) in sums.iter().enumerate(){
            if *sum == 21{
                println!("Player {} wins their hand with a blackjack", i + 1);
                still_playing[i] = false;
            }            
        }
    }
}