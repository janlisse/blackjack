use rand::seq::SliceRandom;
use rand::thread_rng;
use strum_macros::EnumIter;
use strum::{IntoEnumIterator};
use std::io;

#[derive(Debug, Copy, Clone, EnumIter)]
enum Rank {
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
    Ace
}

#[derive(Debug, Copy, Clone, EnumIter)]
enum Suit {
    Spade,
    Heart,
    Diamond,
    Club
}


#[derive(Debug)]
struct Card {
    rank: Rank,
    suit: Suit
}

struct Deck {
    cards: Vec<Card>
}

impl Deck {
    fn new() -> Deck {
    
    let mut cards = Vec::<Card>::new();
    for rank in Rank::iter() {
        for suit in Suit::iter() {
            cards.push(Card {rank, suit});
        }
    }    
    let mut rng = thread_rng();
    cards.shuffle(&mut rng);
    return Deck { cards}
    }

    fn next_card(&mut self) -> Card {
        return self.cards.pop().expect("Cards must not be empty");        
    }
}

fn card_value(card: &Card) -> u8 {
     match card.rank {
        Rank::Two => 2,
        Rank::Three => 3,
        Rank::Four => 4,
        Rank::Five => 5,
        Rank::Six => 6,
        Rank::Seven => 7,
        Rank::Eight => 8,
        Rank::Nine => 9,
        Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        Rank::Ace => 11
    }
}

fn hand_value(cards: &Vec<Card>) -> u8 {
    let mut score:u8 = cards.iter().map(|c| card_value(&c)).sum();
    let mut num_aces = cards.iter().filter(|&c| matches!(c.rank, Rank::Ace)).count();
    let has_aces = num_aces > 0;
    
    while num_aces > 0 {
        if score > 21 && has_aces {
            score -= 10;
            num_aces -= 1;
        }
        else {
            break
        }
    }
    return score;
}


fn main() {

    let mut deck =  Deck::new();
    let mut player_in = true;

    let mut player_hand = vec![deck.next_card(), deck.next_card()];
    let mut dealer_hand = vec![deck.next_card(), deck.next_card()];

    println!("Dealer hand: {:?}", dealer_hand);
    println!("Player hand: {:?}", player_hand);
    
    while player_in {
        let current_score = hand_value(&player_hand);
        println!("Current score: {}", current_score);


        // check if Player is bust
        if current_score > 21 {
            break;
        }

        // ask for another card
        println!("Hit or stay? (Hit=h, Stay=s)");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Wrong option");
        let command = command.trim();
        if command == "h" {
            let card = deck.next_card();
            println!("You draw: {:?}", card);
            player_hand.push(card);
        } else if command == "s" {
            player_in = false;
        }
    }

    let player_score = hand_value(&player_hand);

    while hand_value(&dealer_hand) < 17 && player_score <= 21 {
        let new_dealer_card = deck.next_card();
        println!("Dealer draws {:?}", new_dealer_card);
        dealer_hand.push(new_dealer_card);
    }
    let dealer_score = hand_value(&dealer_hand);

    println!("Final scores. Player: {}, dealer: {}", player_score, dealer_score);

    if player_score > 21 {
        println!("Dealer wins!");
    }
    else if dealer_score > 21 {
        println!("You beat the dealer!");
    }
    else if player_score > dealer_score {
        println!("You beat the dealer!");
    }
    else if player_score == dealer_score {
        println!("Tie, nobody wins.");
    }
    else if player_score < dealer_score {
        println!("Dealer wins!")
    }
}


