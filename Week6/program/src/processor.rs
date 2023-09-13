use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    msg,
    pubkey::Pubkey, 
    account_info::{AccountInfo, next_account_info}, 
    entrypoint::ProgramResult, 
    program_error::ProgramError, 
    rent::Rent, 
    sysvar::Sysvar, 
    program::invoke_signed, 
    system_instruction
};

use crate::{instruction::ExtSplInstruction, state::ExtMint};




pub struct Processor {}

impl Processor {
    pub fn process_mint(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        name: String,
        symbol: String,
        icon: String
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let auth_account = next_account_info(accounts_iter)?;
        let spl_token_program_account = next_account_info(accounts_iter)?;
        let system_program_account = next_account_info(accounts_iter)?;
        let mint_account = next_account_info(accounts_iter)?;
        let ext_mint_account = next_account_info(accounts_iter)?;

        let (gen_ext_mint_key, bump) = Pubkey::find_program_address(
            &[
                &spl_token_program_account.key.to_bytes(),
                &mint_account.key.to_bytes()
            ], 
            program_id);
        
        if gen_ext_mint_key != *ext_mint_account.key {
            msg!("Error: ext_mint_account address does not match seed derivation");
            return Err(ProgramError::InvalidSeeds);
        }

        let ext_mint = ExtMint{
            mint: *mint_account.key,
            name,
            symbol,
            icon
        };
        let ext_mint_data_len = ext_mint.try_to_vec().unwrap().len();

        let rent = Rent::get()?;
        let invoke_seed: &[&[_]] = &[
            &spl_token_program_account.key.to_bytes(),
            &mint_account.key.to_bytes(),
            &[bump]
        ];
        invoke_signed(
            &system_instruction::create_account(
                auth_account.key, 
                ext_mint_account.key, 
                rent.minimum_balance(ext_mint_data_len).max(1), 
                ext_mint_data_len as u64, 
                program_id), 
            &[
                auth_account.clone(),
                ext_mint_account.clone(),
                system_program_account.clone()], 
            &[invoke_seed]
        )?;

        ext_mint.serialize(&mut *ext_mint_account.data.borrow_mut())?;

        Ok(())
    }


    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]
    ) -> ProgramResult {
        msg!("Beginning processing");
        let instruction = ExtSplInstruction::try_from_slice(instruction_data).map_err(
            |_| ProgramError::InvalidInstructionData
        )?;
        msg!("Instruction unpacked");

        match instruction {
            ExtSplInstruction::Mint { name, symbol, icon }
            => {
                Processor::process_mint(program_id, accounts, name, symbol, icon)?;
            }
        }

        Ok(())
    }
}