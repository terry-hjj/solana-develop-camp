use solana_program::pubkey::Pubkey;
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ExtMint {
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub icon: String
}