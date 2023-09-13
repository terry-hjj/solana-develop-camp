
use {
    num_derive::FromPrimitive,
    num_traits::FromPrimitive,
    thiserror::Error,
    solana_program::{
        decode_error::DecodeError,
        program_error::ProgramError,
        program_error::PrintProgramError,
        msg,
    }
};


#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum ExtSplError {
    #[error("Not owned by HelloWorld program")]
    NotOwnedByHelloWorld,
}

pub type ExtSplResult = Result<(), ExtSplError>;

impl From<ExtSplError> for ProgramError {
    fn from(e: ExtSplError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for ExtSplError {
    fn type_of() -> &'static str {
        "ExtSplError"
    }
}

impl PrintProgramError for ExtSplError {
    fn print<E>(&self)
    where E: 'static 
    + std::error::Error 
    + DecodeError<E> 
    + PrintProgramError 
    + FromPrimitive {
        match self {
            ExtSplError::NotOwnedByHelloWorld 
            => msg!("Error: not have the correct program id")
        }
    }
}