use static_assertions::const_assert;
use sunscreen_compiler_common::GraphQueryError;
use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[cfg(feature = "bulletproofs")]
    #[error("Bulletproofs R1CS error: {0:#?}")]
    BulletproofsR1CSError(Box<bulletproofs::r1cs::R1CSError>),

    #[error("Value {0} is out of range for the chosen backend")]
    OutOfRange(Box<String>),

    #[error("The number of inputs passed to an R1CS circuit didn't match the number of inputs to the circuit.")]
    InputsMismatch,

    #[error("The given proof isn't valid for the backend proof system.")]
    IncorrectProofType,

    #[error("The backend graph is malformed {0}")]
    GraphQueryError(#[from] GraphQueryError),
}

impl Error {
    pub fn out_of_range(val: &str) -> Self {
        Self::OutOfRange(Box::new(val.to_owned()))
    }
}

impl From<bulletproofs::r1cs::R1CSError> for Error {
    fn from(e: bulletproofs::r1cs::R1CSError) -> Self {
        Self::BulletproofsR1CSError(Box::new(e))
    }
}

const_assert!(std::mem::size_of::<Error>() <= 16);

pub type Result<T> = std::result::Result<T, Error>;
