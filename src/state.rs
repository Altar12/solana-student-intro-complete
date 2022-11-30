use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_pack::{IsInitialized, Sealed};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct StudentIntroAccountState {
    pub is_initialized: bool,
    pub name: String,
    pub msg: String,
}

impl Sealed for StudentIntroAccountState {}
impl IsInitialized for StudentIntroAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
