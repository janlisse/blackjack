use crate::app::{App, Card, Suit, Rank, Hand, GameResult};

use tui::{
    backend::{Backend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame
};

const NEW_GAME_TEXT: &str = "Press [n] to start a new game";
const NO_SPACE: &str = "";
const SINGLE_SPACE: &str = " ";

const TITLE: &str = r#"
 _____                   _             _   ____  _            _     _            _    
|_   _|__ _ __ _ __ ___ (_)_ __   __ _| | | __ )| | __ _  ___| | __(_) __ _  ___| | __
  | |/ _ \ '__| '_ ` _ \| | '_ \ / _` | | |  _ \| |/ _` |/ __| |/ /| |/ _` |/ __| |/ /
  | |  __/ |  | | | | | | | | | | (_| | | | |_) | | (_| | (__|   < | | (_| | (__|   < 
  |_|\___|_|  |_| |_| |_|_|_| |_|\__,_|_| |____/|_|\__,_|\___|_|\_\/ |\__,_|\___|_|\_\
                                                                 |__/                 
"#;

const DECK: &str = r#"
╭───────╮╮╮╮╮╮
│░░░░░░░││││││
│░░░░░░░││││││
│░░░░░░░││││││
│░░░░░░░││││││
╰───────╯╯╯╯╯╯"#;

const BACK: &str = r#"
╭───────────╮
│░░░░░░░░░░░│
│░░░░░░░░░░░│
│░░░░░░░░░░░│
│░░░░░░░░░░░│
│░░░░░░░░░░░│
│░░░░░░░░░░░│
│░░░░░░░░░░░│
╰───────────╯
"#;

fn card_face(rank: &str, suit: &str, spacing: &str) -> String { return format!(
      "\n╭───────────╮\n\
         │{rank}{spacing}         │\n\
         │           │\n\
         │           │\n\
         │     {suit}     │\n\
         │           │\n\
         │           │\n\
         │         {spacing}{rank}│\n\
         ╰───────────╯", rank=rank, spacing=spacing, suit=suit);
}


pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let block = Block::default();
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(10), Constraint::Min(40)].as_ref())
        .split(f.size());

    
    render_title(f,chunks[0]);
    
    // Game table
    let block = Block::default()
        .style(Style::default().fg(Color::Black).bg(Color::White));
    f.render_widget(block, chunks[1]);

    let card_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(40), Constraint::Percentage(40)].as_ref())
        .split(chunks[1]);

    render_status(f, card_chunks[0], app);
    render_hand(f, card_chunks[1], &app.dealer_hand, Color::Red, "Dealer");
    render_hand(f, card_chunks[2], &app.player_hand, Color::Black, "Player");
    
}

fn render_title<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let caption_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .margin(1)
    .constraints([Constraint::Min(15), Constraint::Min(86)].as_ref())
    .split(area);

    let deck_logo = Paragraph::new(DECK)
                .alignment(Alignment::Left);
    
    f.render_widget(deck_logo, caption_chunks[0]);

    let caption = Paragraph::new(TITLE)
                .alignment(Alignment::Left);
                

    f.render_widget(caption, caption_chunks[1]);
}

fn render_hand<B: Backend>(f: &mut Frame<B>, area: Rect, hand: &Hand, color: Color, name: &str) {
    let num_cards = u16::try_from(hand.cards.len()).unwrap();
    if num_cards > 0 {
        let constraints = get_min_constraints(num_cards);
        let card_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(constraints.as_ref())
            .split(area);
        for (i, card) in hand.cards.iter().enumerate() {
            let is_first = i == 0;
            render_card(f, card_chunks[i], card, color, if is_first {Some(name)} else {None});
        }
    }
}

fn render_card<B: Backend>(f: &mut Frame<B>, area: Rect, card: &Card, color: Color, name: Option<&str>) {
    let suit = match card.suit {
        Suit::Spade => "♠",
        Suit::Club => "♣",
        Suit::Diamond => "♦",
        Suit::Heart => "♥"
    };

    let rank = match card.rank {
        Rank::Two => "2",
        Rank::Three => "3",
        Rank::Four => "4",
        Rank::Five => "5",
        Rank::Six => "6",
        Rank::Seven => "7",
        Rank::Eight => "8",
        Rank::Nine => "9",
        Rank::Ten => "10",
        Rank::Jack => "J",
        Rank::Queen => "Q",
        Rank::King => "K",
        Rank::Ace => "A",
    };

    let spacing = match card.rank {
        Rank::Ten => NO_SPACE,
        _ => SINGLE_SPACE
    };

    let title = match name {
        Some(name) => name,
        None => ""
    };

    let card_face = if card.hide { BACK.to_string() }  else { card_face(rank,suit, spacing) };

    let card = Paragraph::new(card_face)
        .block(Block::default().style(Style::default()
        .fg(color)).title(title))
        .alignment(Alignment::Left);
    f.render_widget(card, area);
}

fn render_status<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    if !app.game_running {
        let status_bar = Paragraph::new(NEW_GAME_TEXT)
                .alignment(Alignment::Center);
        f.render_widget(status_bar, area);
    }
    if app.players_turn {
        let status_bar = Paragraph::new("Draw card [d] or stand [s]?")
                .alignment(Alignment::Center);
        f.render_widget(status_bar, area);
    }
    if let Some(game_result) = &app.result {
        let text = match game_result {
            GameResult::Won => "Congrats, you won the game!",
            GameResult::Lost => "Sorry, you lost the game!",
            GameResult::Tie => "Nobody won, it's a tie.",
            GameResult::Bust => "You got bust!"
        };
        let status_bar = Paragraph::new(format!("{}\n{}", text, NEW_GAME_TEXT))
                .alignment(Alignment::Center);
        f.render_widget(status_bar, area);
    }
}

fn get_min_constraints(num: u16)-> Vec<Constraint> {
    return (1..=num).map(|_| Constraint::Length(13)).collect::<Vec<Constraint>>();
}