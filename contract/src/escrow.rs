use near_sdk::{
    env,
    near_bindgen,
    AccountId,
    Balance,
};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct EscrowContract {
    escrow_amount: Balance,
    sender: AccountId,
    receiver: AccountId,
    arbiter: AccountId,
    finalized: bool,
}

#[near_bindgen]
impl EscrowContract {
    pub fn new(escrow_amount: Balance, sender: AccountId, receiver: AccountId, arbiter: AccountId) -> Self {
        Self { 
            escrow_amount,
            sender,
            receiver,
            arbiter,
            finalized: false
        }
    }

    pub fn release_funds(&mut self) {
        assert!(env::predecessor_account_id() == self.arbiter, "Only the arbiter can release funds.");
        assert!(!self.finalized, "Funds have already been released.");
        self.finalized = true;
        env::transfer_to_account(self.receiver, self.escrow_amount);
    }

    pub fn get_escrow_amount(&self) -> Balance {
        self.escrow_amount
    }

    pub fn get_sender(&self) -> AccountId {
        self.sender
    }

    pub fn get_receiver(&self) -> AccountId {
        self.receiver
    }

    pub fn get_arbiter(&self) -> AccountId {
        self.arbiter
    }

    pub fn is_finalized(&self) -> bool {
        self.finalized
    }
}
