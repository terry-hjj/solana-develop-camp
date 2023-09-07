use {
    crate::error::HelloWorldError,
    crate::processor::Processor,
    num_traits::FromPrimitive,
    solana_program::{
        account_info::AccountInfo, decode_error::DecodeError, entrypoint,
        entrypoint::ProgramResult, msg, program_error::PrintProgramError, pubkey::Pubkey,
    },
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("entrypoint");
    if let Err(error) = Processor::process_instruction(program_id, accounts, instruction_data) {
        error.print::<HelloWorldError>();
        return Err(error);
    }
    Ok(())
}
