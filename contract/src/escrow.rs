use near_sdk::{ call, LookupMap, NearBindgen, view, assert, near, UnorderedMap, NearPromise };

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Escrow {}