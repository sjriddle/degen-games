use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::{log, near_bindgen, AccountId, Timestamp, env};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Wager {
    game: GameType,
    // Creator of the wager
    challenger: AccountId,
    // Amount to be locked by each party
    amount: U128,
    state: WagerState,
    // Opponent of the game
    opponent: Option<AccountId>,
    // Have the challenger & opponent staked the wager amount
    challenger_staked: bool,
    opponent_staked: bool,
    // Expiration is 3 days as default
    expiration: Timestamp,
}

#[near_bindgen]
pub enum GameType {
    Chess,
    Checkers,
}

/// Created - State that the initial wager comes in with the Challenger's parameters
/// Confirming - Confirming that there is an opponent and that they agree to the same amount
/// Pending - Both parties have staked full amount into escrow but game has NOT begun
/// Initiated - Game has started and conclusion has not been reached
/// Finished  - The results of the game have been determined
/// Cancelled - The Challenger cancelled the wager
/// Expired   - Wager has expired past default expiration time period of 3 daysf
#[near_bindgen]
pub enum WagerState {
    Created,
    Confirming,
    Pending,
    Initiatied,
    Finished,
    Cancelled,
    Expired
}

#[near_bindgen]
impl Wager {

    pub fn create_wager(&self, game: GameType, amount: U128, opponent: AccountId) {
        Self {
            game,
            challenger: env::predecessor_account_id,
            amount,
            state: WagerState::Created,
            challenger_staked: false,
            opponent_staked: false,
            opponent
        }
    }

    pub fn cancel_wager(&mut self) {
        assert!(self.wager.state == WagerState::Created,))
    }
}


// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
}