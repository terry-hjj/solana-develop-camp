use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        borsh::try_from_slice_unchecked,
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum HelloWorldInstruction {
    Create(String),
    Modify(String),
    Delete,
}
