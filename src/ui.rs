use crate::app::{App, Card, Suit, Rank, Hand, GameResult};

use tui::{
    backend::{Backend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Paragraph},
    Frame
};

const NO_SPACE: &str = "";
const SINGLE_SPACE: &str = " ";

const DECK: &str = 
   "╭───────────╮╮╮╮╮╮\n\
    │░░░░░░░░░░░││││││\n\
    │░░░░░░░░░░░││││││\n\
    │░░░░░░░░░░░││││││\n\
    │░░░░░░░░░░░││││││\n\
    │░░░░░░░░░░░││││││\n\
    │░░░░░░░░░░░││││││\n\
    │░░░░░░░░░░░││││││\n\
    ╰───────────╯╯╯╯╯╯";

const BACK: &str = 
   "╭───────────╮\n\
    │░░░░░░░░░░░│\n\
    │░░░░░░░░░░░│\n\
    │░░░░░░░░░░░│\n\
    │░░░░░░░░░░░│\n\
    │░░░░░░░░░░░│\n\
    │░░░░░░░░░░░│\n\
    │░░░░░░░░░░░│\n\
    ╰───────────╯";

fn card_face(rank: &str, suit: &str, spacing: &str) -> String { return format!(
        "╭───────────╮\n\
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
        .constraints([Constraint::Percentage(5), Constraint::Percentage(85), Constraint::Percentage(10)].as_ref())
        .split(f.size());

    let text = Span::styled(
        "Terminal Blackjack",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD));    
    
    let caption = Paragraph::new(text)
                .alignment(Alignment::Center);
                

    f.render_widget(caption, chunks[0]);

    
    // Game table
    let block = Block::default()
        .style(Style::default().fg(Color::Black).bg(Color::White));
    f.render_widget(block, chunks[1]);

    let card_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)].as_ref())
        .split(chunks[1]);

    let deck = Paragraph::new(DECK)
        .block(Block::default().style(Style::default()
        .fg(Color::Black)))
        .alignment(Alignment::Left);

    f.render_widget(deck, card_chunks[0]);

    render_hand(f, card_chunks[1], &app.dealer_hand, "Dealer");
    render_hand(f, card_chunks[2], &app.player_hand, "Player");
    render_status(f, chunks[2], app);
}

fn render_hand<B: Backend>(f: &mut Frame<B>, area: Rect, hand: &Hand, name: &str) {
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
            render_card(f, card_chunks[i], card, if is_first {Some(name)} else {None});
        }
    }
}

fn render_card<B: Backend>(f: &mut Frame<B>, area: Rect, card: &Card, name: Option<&str>) {
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
        .fg(Color::Black)).title(title))
        .alignment(Alignment::Left);
    f.render_widget(card, area);
}

fn render_status<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    if !app.game_running {
        let status_bar = Paragraph::new("Press [n] to start a new game!")
                .alignment(Alignment::Center);
        f.render_widget(status_bar, area);
    }
    if app.ask_for_card {
        let status_bar = Paragraph::new("Draw card [d] or stay [s]?")
                .alignment(Alignment::Center);
        f.render_widget(status_bar, area);
    }
    if let Some(game_result) = &app.result {
        let text = match game_result {
            GameResult::Won => "Congrats, you won the game!",
            GameResult::Lost => "Sorry, you lost the game..",
            GameResult::Tie => "Boring, it's a tie."
        };
        let status_bar = Paragraph::new(text)
                .alignment(Alignment::Center);
        f.render_widget(status_bar, area);
    }
}

fn get_min_constraints(num: u16)-> Vec<Constraint> {
    return (1..=num).map(|_| Constraint::Length(13)).collect::<Vec<Constraint>>();
}

fn get_constraints(num: u16) -> Vec<Constraint> {
    let d = 100 / num;
    let r = 100 % num;

    let mut s1 = (1..=r).map(|n| Constraint::Percentage(d+1)).collect::<Vec<Constraint>>();
    let mut s2 = (1..=(num-r)).map(|n| Constraint::Percentage(d)).collect::<Vec<Constraint>>();

    s1.append(&mut s2);
    return s1;
}