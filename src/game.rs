/*!
 * This module defines a game of colour whist, which consists of tricks and the current table
 */

use std::{thread, time::Duration};

use crate::{
    deck::Deck,
    card::Card,
    player::Player, suit::Suit, fortify::GameState,
};
use text_io::read;

#[derive(Hash, Eq, PartialEq)]
pub struct Game {
    /// tricks keeps track of the played tricks
    /// one trick is a deck of 4 cards
    tricks: Vec<Deck>,
    /// table keeps track of which cards are 
    /// on the table already
    table: Deck,
    pub players: [Player; 4],
    turn: usize,
    trump: Suit,
    scores: [u32; 4]
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Game {
        let mut deck = Deck::new_full();
        deck.shuffle();
        let tricks: Vec<Deck> = Vec::new();
        let table: Deck = Deck::new_empty();
        
        // create new players
        let player_one = Player::new_take_cards(&mut deck, 13);
        let player_two = Player::new_take_cards(&mut deck, 13);
        let player_three = Player::new_take_cards(&mut deck, 13);
        let player_four = Player::new_take_cards(&mut deck, 13);
        // deck is empty now

        let players = [player_one, player_two, player_three, player_four];
        let scores = [0,0,0,0];

        Game{ tricks, table, players, turn: 0 , trump: Suit::Hearts, scores}
    }

    pub fn new_round(&mut self) {
        let mut deck = Deck::new_full();
        deck.shuffle();

        let cards = deck.pull_cards(13);
        self.players[0].cards = cards;

        let cards = deck.pull_cards(13);
        self.players[1].cards = cards;

        let cards = deck.pull_cards(13);
        self.players[2].cards = cards;

        let cards = deck.pull_cards(13);
        self.players[3].cards = cards;

        // self.turn = 0;
    }

    fn show_last_trick(&self) {
        if !self.tricks.is_empty() {
            println!("Played trick:\n{}\n",self.tricks[self.tricks.len()-1]);
        }
    }

    pub fn trick(&mut self) -> Result<(), String> {
        if self.table.size() == 4 {
            let new_trick = self.table.pull_cards(4);
            self.tricks.push(new_trick);
            Ok(())
        } else {
            Err("There are not exactly four cards on the table.".to_string())
        }
    }

    /// Put the card on the table.
    /// Comsumes the card!
    fn play(&mut self, card: Card) -> Result<(), String> {
        if self.table.size() < 4 {
            self.table.add(card);
            Ok(())
        } else {
            Err(String::from("There are already four cards on the table. Can't play any more."))
        }
    }

    pub fn show_table(&self) {
        self.table.show();
    }

    pub fn play_card(&mut self, player: usize, card: usize) {
        let played = self.players[player].cards.remove(card);
        self.play(played).expect("Couldn't play card");
    }

    /// returns the winning card index currently on the table
    fn winner(&self) -> usize {
        let mut best_card = self.table.card(0);
        let mut winner = 0;

        for i in 1..self.table.size() {
            let new_card = self.table.card(i);

            if new_card.better(best_card, &self.trump) {
                best_card = new_card;
                winner = i;
            }
        }

        winner
    }

    /// returns a vector of alowed cards for this player, in this round
    fn alowed_cards(&self, player: usize) -> Vec<usize> {
        // if the table is empty, every card is alowed
        let mut result: Vec<usize> = Vec::new();

        let player = &self.players[player];

        if self.table.size() != 0 { // i'm possibly restricted to the first-layed card this trick
            let first_suit = self.table.card(0).suit;

            if player.cards.has_suit(&first_suit) {
                // only return indexes of cards of said suit
                for i in 0..player.cards.size() {
                    if player.cards.suit_at(i) == first_suit {
                        result.push(i);
                    }
                }
                return result;
            }
        }

        // if I got here, i am free to play any card
        for i in 0..player.cards.size() {
            result.push(i);
        }

        result    
    }

    fn my_better_cards(&self, player: usize, playable: &[usize]) -> Vec<usize> {
        if self.table.size() == 0 {
            return playable.to_vec();
        }

        let player = &self.players[player];
        let best_on_table = self.winner();
        let best_on_table = self.table.card(best_on_table);

        playable.iter().cloned()
                .filter(|card| player.cards.card(*card).better(best_on_table, &self.trump))
                .collect()
    }

    fn play_easy(&mut self, player: usize) {
        // TODO improve AI significantly
        // -> using REINFORCEMENT LEARNING CRATE
    
        // get alowed indeces
        let playable = self.alowed_cards(player);

        let better_cards = self.my_better_cards(player, &playable);

        if !better_cards.is_empty() {
            return self.play_card(player, self.players[player].cards.lowest(&better_cards, &Suit::Hearts));
        }

        // play other card
        self.play_card(player, self.players[player].cards.lowest(&playable, &Suit::Hearts));
    }

    fn show_table_wait(&self) {
        println!("\x1b[1J\x1b[H");
        println!("Current table: \n{}\n", self.table);
        thread::sleep(Duration::from_millis(500));
    }

    pub fn play_round(&mut self) {
        for i in self.turn..self.turn+4 {
            let player = i % 4;

            if player == 0 {
                self.show_table_wait();
                self.show_last_trick();
                println!("Your hand:");
                self.players[0].show_cards();

                let playable = self.alowed_cards(player);

                let idx: usize = loop {
                    // loop until correct card given
                    println!("Enter a suit (S,C,D,H):");
                    let suit: String = read!();
                    
                    let suit = match suit.as_str() {
                        "H" | "h" => Suit::Hearts,
                        "D" | "d" => Suit::Diamonds,
                        "C" | "c" => Suit::Clubs,
                        "S" | "s" => Suit::Spades,
                        &_ => Suit::Hearts,
                    };
                    
                    println!("Enter a value (1-13):");
                    let number: u8 = read!();
                    
                    let card = Card{suit, number};
                    
                    if let Some(i) = self.players[0].cards.index_of(&card) {
                        if playable.contains(&i) {
                            break i;
                        } 
                    }
                    println!("Try again!\n");
                };

                self.play_card(0, idx);

                self.show_table_wait();
            } else {
                // AI thinks for a little while
                // thread::sleep(Duration::from_millis(500));
                
                self.play_easy(player);

                self.show_table_wait();
            }
        }

        self.turn = (self.winner() + self.turn) % 4;
        self.scores[self.turn] += 1;

        println!("Winner this round: player {}", self.turn);
        thread::sleep(Duration::from_millis(1500));

        self.trick().expect("Couldn't play trick in play_round");
    }

    pub fn show_scores(&self) {
        println!("The scores: {:?}", self.scores);
    }

    pub fn reward(&self) -> u32 {
        self.scores[0]
    }

    pub fn state(&self) -> GameState {
        GameState::new()
    }
}