use {
    num_derive::FromPrimitive,
    num_traits::FromPrimitive,
    solana_program::{
        decode_error::DecodeError, msg, program_error::PrintProgramError,
        program_error::ProgramError,
    },
    thiserror::Error,
};

#[derive(Clone, Debug, Eq, PartialEq, Error, FromPrimitive)]
pub enum HelloWorldError {
    #[error("Not owned by HelloWorld Program")]
    NoOwnedByHelloWorld,
}

pub type HelloWorldResult = Result<(), HelloWorldError>;

impl From<HelloWorldError> for ProgramError {
    fn from(e: HelloWorldError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for HelloWorldError {
    fn type_of() -> &'static str {
        "HelloWorldError"
    }
}

impl PrintProgramError for HelloWorldError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            HelloWorldError::NoOwnedByHelloWorld => {
                msg!("Error: Greeted account does not have the correct program id")
            }
        }
    }
}
