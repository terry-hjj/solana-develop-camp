
use borsh::{BorshSerialize, BorshDeserialize, BorshSchema};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq, BorshSchema)]
pub enum ExtSplInstruction {
    Mint {
        name: String,
        symbol: String,
        icon: String
    }
}