use core::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand;
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
pub(crate) struct Card {
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
pub(crate) struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new_hand() -> Hand{
        Hand { cards: Vec::new() }
    }

    pub fn add_card(&mut self, card: Card){
        self.cards.push(card);
    }

    pub fn calculate_sum(&self) -> i32{
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

    pub fn print_hand(&self){
        for card in self.cards.iter() {
            card.print_card();
            print!(", ");
        }
    }


    pub fn print_dealer_hand(&self){
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
pub(crate) struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn shuffle(&mut self){
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
    pub fn deal_card(&mut self) -> Card{
        let dealt_card = self.cards.pop();
        match dealt_card{
            Some(card) => return card,
            None => panic!("No cards in deck to deal"),
        }
    }

}