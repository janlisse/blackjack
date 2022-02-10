use rand::seq::SliceRandom;
use rand::thread_rng;
use strum_macros::EnumIter;
use strum::{IntoEnumIterator};

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum Rank {
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
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club
}

pub enum GameResult {
    Won,
    Lost,
    Tie
}


#[derive(Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub hide: bool
}

impl Card {
    fn score(&self) -> u8 {
        match self.rank {
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
}

pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    fn score(&self) -> u8 {
        let mut score:u8 = self.cards.iter().map(|c| c.score()).sum();
        let mut num_aces = self.cards.iter().filter(|&c| matches!(c.rank, Rank::Ace)).count();
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

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    } 
}

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    fn new() -> Deck {
    
    let mut cards = Vec::<Card>::new();
    for rank in Rank::iter() {
        for suit in Suit::iter() {
            cards.push(Card {rank, suit, hide: false});
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

pub struct App {
    pub deck: Deck,
    pub game_running: bool,
    pub player_in: bool,
    pub ask_for_card: bool,
    pub player_hand: Hand,
    pub dealer_hand: Hand,
    pub result: Option<GameResult>
}

impl App {
    pub fn new() -> App {
        App {
            deck: Deck::new(),
            player_hand: Hand{cards:vec![]},
            dealer_hand: Hand{cards:vec![]},
            game_running: false,
            player_in: true,
            ask_for_card: false,
            result: None
        }
    }

    pub fn on_start(&mut self) {
        self.game_running = true;
        self.deck = Deck::new();
        self.player_hand = Hand{cards: vec![self.deck.next_card(), self.deck.next_card()]};
        self.dealer_hand = Hand{cards:vec![self.deck.next_card(), self.deck.next_card()]};
        self.dealer_hand.cards[0].hide = true;
        self.ask_for_card = true
    }
    
    pub fn on_draw(&mut self) {
        if self.ask_for_card {
            let card = self.deck.next_card();
            self.player_hand.add(card);

            let current_score = self.player_hand.score();
            if current_score > 21 {
                self.player_in = false;
                self.ask_for_card = false
            }
        }
    }

    pub fn on_stay(&mut self) {
        if self.ask_for_card {
            self.player_in = false;
            self.ask_for_card = false
        }
    }

    pub fn on_tick(&mut self) {
        if !self.player_in {
            let player_score = self.player_hand.score();
            let dealer_score = self.dealer_hand.score();
            self.dealer_hand.cards[0].hide = false;
            if dealer_score < 17 && player_score <= 21 {
                let new_card = self.deck.next_card();
                self.dealer_hand.add(new_card);
            } else {
                // round is over
                if player_score > 21 {
                    self.result = Some(GameResult::Lost);
                }
                else if dealer_score > 21 {
                    self.result = Some(GameResult::Won);
                }
                else if player_score > dealer_score {
                    self.result = Some(GameResult::Won);
                }
                else if player_score == dealer_score {
                    self.result = Some(GameResult::Tie);
                }
                else if player_score < dealer_score {
                    self.result = Some(GameResult::Lost);
                }
            }
        }
    }
}

