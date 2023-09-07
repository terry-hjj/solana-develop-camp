use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

pub struct Processor {}

impl Processor {
    pub fn process_create(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        msg: String,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let greeting_account = next_account_info(account_iter)?;
        if greeting_account.owner != program_id {
            msg!("Greeted account does not have the correct program id");
            return Err(HelloWorldError::NoOwnedByHelloWorld.into());
        }
        let mut greeting_info = GreetingInfo {
            message: "".to_string(),
        };
        greeting_info.message = msg;
        greeting_info.serialize(&mut *greeting_account.data.borrow_mut())?;
        msg!("set note to {} !", greeting_info.message);
        Ok(())
    }
    pub fn process_modify(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        msg: String,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let greeting_account = next_account_info(accounts_iter)?;
        if greeting_account.owner != program_id {
            msg!("Greeted account does not have the correct program id");
            return Err(HelloWorldError::NoOwnedByHelloWorld.inot());
        }
        let mut greeting_info = GreetingInfo::try_from_slice(&greeting_account.data.borrow())?;
        greeting_info.message = msg;
        greeting_info.serialize(&mut *greeting_account.data.borrow_mut())?;
        msg!("set note to {} !", greeting_info.message);
        Ok(())
    }
    pub fn process_delete(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let user_account = next_account_info(accounts_iter)?;
        let greeting_account = next_account_info(accounts_iter)?;
        if greeting_account.owner != program_id {
            msg!("Greeted account does not have the correct program id");
            return Err(HelloWorldError::NoOwnedByHelloWorld.inot());
        }
        **user_account.try_borrow_mut_lamports()? += greeting_account.lamports();
        **greeting_account.try_borrow_mut_lamports()? = 0;
        Ok(())
    }
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Beginning processing");
        let instruction = HelloWorldInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        msg!("Instruction unpacked");

        match instruction {
            HelloWorldInstruction::Create(msg) => {
                Processor::process_create(program_id, accounts, msg)?;
            }
            HelloWorldInstruction::Modify(msg) => {
                Processor::process_modify(program_id, accounts, msg)?;
            }
            HelloWorldInstruction::Delete => {
                Processor::process_delete(program_id, accounts)?;
            }
        }
        Ok(())
    }
}
