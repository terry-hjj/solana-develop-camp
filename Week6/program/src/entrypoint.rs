use solana_program::{
    msg, 
    pubkey::Pubkey, 
    account_info::AccountInfo, 
    entrypoint,
    entrypoint::ProgramResult, 
    program_error::PrintProgramError
};

use crate::{error::ExtSplError, processor::Processor};



entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    msg!("EntryPoint");
    if let Err(error) = Processor::process_instruction(program_id, accounts, instruction_data) {
        error.print::<ExtSplError>();
        return Err(error);
    }
    Ok(())
}