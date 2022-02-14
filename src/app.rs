use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
    Ace,
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

pub enum GameResult {
    Won,
    Lost,
    Push,
}

#[derive(PartialEq)]
pub enum PlayerStatus {
    Bust,
    Draw,
    Blackjack,
    Stand,
}

#[derive(Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub hidden: bool,
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
            Rank::Ace => 11,
        }
    }
}

pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    fn score(&self) -> u8 {
        let mut score: u8 = self.cards.iter().map(|c| c.score()).sum();
        let mut num_aces = self
            .cards
            .iter()
            .filter(|&c| matches!(c.rank, Rank::Ace))
            .count();
        let has_aces = num_aces > 0;
        while num_aces > 0 {
            if score > 21 && has_aces {
                score -= 10;
                num_aces -= 1;
            } else {
                break;
            }
        }
        return score;
    }

    fn has_blackjack(&self) -> bool {
        return self.cards.len() == 2 && self.score() == 21;
    }

    fn is_bust(&self) -> bool {
        return self.score() > 21;
    }

    fn has_hidden(&self) -> bool {
        return self.cards.iter().filter(|c| c.hidden).count() > 0;
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn empty() -> Deck {
        return Deck { cards: vec![] };
    }

    fn shuffle(&mut self) {
        let mut cards = Vec::<Card>::new();
        for rank in Rank::iter() {
            for suit in Suit::iter() {
                cards.push(Card {
                    rank,
                    suit,
                    hidden: false,
                });
            }
        }
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);
        self.cards = cards;
    }

    fn next_card(&mut self) -> Card {
        return self.cards.pop().expect("Cards must not be empty");
    }
}

pub struct App {
    pub deck: Deck,
    pub game_running: bool,
    pub player_hand: Hand,
    pub dealer_hand: Hand,
    pub player_status: PlayerStatus,
    pub result: Option<GameResult>,
}

impl App {
    pub fn new() -> App {
        App {
            deck: Deck::empty(),
            player_hand: Hand { cards: vec![] },
            dealer_hand: Hand { cards: vec![] },
            player_status: PlayerStatus::Draw,
            game_running: false,
            result: None,
        }
    }

    pub fn on_start(&mut self) {
        self.game_running = true;
        self.deck.shuffle();
        self.result = None;
        self.player_hand = Hand {
            cards: vec![self.deck.next_card(), self.deck.next_card()],
        };
        self.dealer_hand = Hand {
            cards: vec![self.deck.next_card(), self.deck.next_card()],
        };
        self.player_status = PlayerStatus::Draw;
        self.dealer_hand.cards[0].hidden = true;

        if self.player_hand.has_blackjack() {
            self.player_status = PlayerStatus::Blackjack;
        }
    }
    pub fn on_draw(&mut self) {
        if self.player_status == PlayerStatus::Draw {
            let card = self.deck.next_card();
            self.player_hand.add(card);

            if self.player_hand.is_bust() {
                self.player_status = PlayerStatus::Bust;
            }
        }
    }

    pub fn on_stay(&mut self) {
        if self.player_status == PlayerStatus::Draw {
            self.player_status = PlayerStatus::Stand
        }
    }

    pub fn on_tick(&mut self, counter: u64) {
        if self.player_hand.is_bust() {
            self.result = Some(GameResult::Lost);
        } else if self.dealer_hand.is_bust() {
            self.result = Some(GameResult::Won);
        } else if !(self.player_status == PlayerStatus::Draw) && App::delay(counter) {
            let player_score = self.player_hand.score();
            let dealer_score = self.dealer_hand.score();

            if self.dealer_hand.has_hidden() {
                self.dealer_hand.cards[0].hidden = false;
            } else if dealer_score < 17 {
                let new_card = self.deck.next_card();
                self.dealer_hand.add(new_card);
            } else {
                if self.player_hand.has_blackjack() && self.dealer_hand.has_blackjack() {
                    self.result = Some(GameResult::Push);
                } else if self.dealer_hand.has_blackjack() || player_score < dealer_score {
                    self.result = Some(GameResult::Lost);
                } else if self.player_hand.has_blackjack() || player_score > dealer_score {
                    self.result = Some(GameResult::Won);
                } else if player_score == dealer_score {
                    self.result = Some(GameResult::Push);
                }
            }
        }
    }
    fn delay(counter: u64) -> bool {
        counter % 4 == 0
    }
}
