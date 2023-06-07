/*!
 * This module implements approximate Q-learning for the whister card game.
 *
 * Multiple difficulty levels are defined, by giving the agent increasing amounts of data to
 * train on. Trained models will be supplied when they are finished (basically a serialized Q hash map).
 */
#![allow(unused_variables)]

use crate::game::Game;
use std::collections::HashMap;


/// All possible actions that the agent can take.
/// Technically, the action could be described as just a Card, but
/// these are realistic moves for many scenarios, to reduce the state-action space.
#[derive(Hash, PartialEq, Eq)]
pub enum Action {
    /// play the lowest card you can
    PlayWorst,
    /// play a higher card, but the lowest you can
    RaiseLow,
    /// play a higher card, the highest you can
    RaiseHigh,
    /// play the highest trump card you can
    BuyHigh,
    /// play the lowest trump card that buys it
    BuyLow,
}

#[derive(PartialEq, Eq, Hash, Clone, Default)]
pub struct GameState {}

impl GameState {
    pub fn new() -> GameState {
        GameState {}
    }
}

type Q = HashMap<GameState, HashMap<Action, f64>>;

pub struct QLearner {
    q: Q,
    rate: f64,
    discount: f64,
    initial_value: f64,
}

impl QLearner {
    pub fn train(&mut self, game: &Game) {
        let current_state = game.state();
        let current_values = self.q.get(&current_state).unwrap_or(&HashMap::new());

        // determine a new action to take, from current state
        let action = self.new_action(&current_state);
        
        // reward is the reward that's coupled with this action
        let reward = game.reward() as f64;
        // best_future is the highest Q-value from the state after taking this action
        let best_future = 0.0;

        // new value to assign to Q(s,a)
        let v: f64 = {
            // get the old value of Q(s,a) if it is available
            let old_value = self.q.get(&current_state).and_then(|m| m.get(&action)).unwrap_or(&self.initial_value);

            *old_value + self.rate * (reward + self.discount * best_future - *old_value)
        };

        self.q
            .entry(current_state)
            .or_insert_with(HashMap::new)
            .insert(action, v);
    }

    /// determine the best action in current state, based on the q function
    pub fn best_action(&self, state: &GameState) -> Action {
        // TODO
        Action::RaiseHigh
    }

    /// determine the action the agent takes while exploring the statespace
    fn new_action(&self, state: &GameState) -> Action {
        // TODO
        Action::PlayWorst
    }
}
