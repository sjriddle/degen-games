use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::{log, near_bindgen, AccountId, Timestamp};


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {

}

#[near_bindgen]
impl Contract {

}


// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
}