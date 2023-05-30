use core::fmt;
use std::fs;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand;
use text_io::read;

#[derive(Debug, EnumIter, Clone, Copy)]
enum Suit {
    Spade,
    Heart,
    Diamond,
    Club
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Spade => write!(f, "Spades"),
            Suit::Heart => write!(f, "Hearts"),
            Suit::Diamond => write!(f, "Diamonds"),
            Suit::Club => write!(f, "Clubs")
        }
    }
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::Ace => write!(f, "Ace"),
            Rank::Two => write!(f, "Two"),
            Rank::Three => write!(f, "Three"),
            Rank::Four => write!(f, "Four"),
            Rank::Five => write!(f, "Five"),
            Rank::Six => write!(f, "Six"),
            Rank::Seven => write!(f, "Seven"),
            Rank::Eight => write!(f, "Eight"),
            Rank::Nine => write!(f, "Nine"),
            Rank::Ten => write!(f, "Ten"),
            Rank::Jack => write!(f, "Jack"),
            Rank::Queen => write!(f, "Queen"),
            Rank::King => write!(f, "King"),
        }
    }
}


#[derive(Clone)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn get_value(&self) -> i32{
        match self.rank{
            Rank::Ace => 11,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
        }
    }

    fn print_card(&self){
        print!("{} of {}", self.rank, self.suit);
    }
}

#[derive(Clone)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new_hand() -> Hand{
        Hand { cards: Vec::new() }
    }

    fn add_card(&mut self, card: Card){
        self.cards.push(card);
    }

    fn calculate_sum(&self) -> i32{
        let mut sum = Vec::new();
        for card in self.cards.iter(){
            sum.push(card.get_value());
        }
        let mut total: i32 = sum.iter().sum();
        if total > 21 && sum.contains(&11){
            for (i, card) in self.cards.iter().enumerate(){
                if let Rank::Ace = card.rank{
                    sum[i] = 1
                }
                total = sum.iter().sum();
                if total <= 21 {
                    break;
                }
            }
        }
        total
    }

    fn print_hand(&self){
        for card in self.cards.iter() {
            card.print_card();
            print!(", ");
        }
    }


    fn print_dealer_hand(&self){
        let first_card = match self.cards.first() {
            Some(card) => card,
            None => panic!("Dealer has no cards"),
        };
        print!("The dealer's visible card is the ");
        first_card.print_card();
        println!();
    }
}

#[derive(Clone)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn shuffle(&mut self){
        let mut new_deck: Vec<Card> = Vec::new();
        while self.cards.len() > 0{
            let index = (rand::random::<f32>() * self.cards.len() as f32).floor() as usize;
            new_deck.push(self.cards.remove(index))
        }
        self.cards = new_deck;
    }

    pub fn set_deck() -> Self{
        let mut new_deck = Deck{cards: Vec::new()};
        for rank in Rank::iter(){
            for suit in Suit::iter(){
                new_deck.cards.push(Card{rank: rank, suit: suit})
            }
        }
        new_deck.shuffle();
        new_deck


    }
    fn deal_card(&mut self) -> Card{
        let dealt_card = self.cards.pop();
        match dealt_card{
            Some(card) => return card,
            None => panic!("No cards in deck to deal"),
        }
    }

}

#[derive(serde_derive::Deserialize)]
struct Variables {
    number_of_players: i32,
    number_of_shuffles: i32,
}

fn main() {
    let file_name = "./src/variables.toml";
    let contents = match fs::read_to_string(file_name){
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
            println!("Dealer wins with a natural blackjack");
            break;
        }
        for (i, hand:&mut Hand) in players.iter().enumerate(){
            if still_playing[i] == false{
                continue;
            }
            let mut turn:String;
            println!("Player {}'s Turn!", i + 1);
            loop {
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
        

    }

}

fn check_for_blackjack(sums:&Vec<i32>, still_playing: &mut Vec<bool>) {
    if sums.contains(&21){
        for (i, sum) in sums.iter().enumerate(){
            if *sum == 21{
                println!("Player {} wins their hand with a natural blackjack", i + 1);
                still_playing[i] = false;
            }            
        }
    }
}