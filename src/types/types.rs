use ethereum_types::{Address, H256};
use serde::{Deserialize, Serialize};

//custom printing of datatype
#[derive(Debug)]
pub struct Logger{
    pub hashes: Vec<H256>,
    pub data: Vec<u8>,
}

//Basic params for evm
#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Env {
    pub caller: Address,
    pub timestamp: U256,
    pub number: U256,
    pub chainid: U256,
    //need hex values for the last 2 params
    #[serde_as(as = "serde_with::hex::Hex")]
    pub calldata: Vec<u8>,
    pub value: U256, 
}

#[derive(PartialEq, Debug)]
pub enum Error {
    StackIsOverflown,
    CodeOutOfBounds,
    InvalidOpcode(u8),
    MemoryIsOverflown,
    MemoryIsOutOfBounds,
    StateStoringError,
}

#[derive(PartialEq, Debug)]
pub enum OpcodeStepping {
    Continue,
    Return(Vec<u8>),
}

pub type OpcodeResult = Result<OpcodeStepping, Error>;

pub type RunResult = Result<(Vec<u8>, Vec<Logger>), Error>;

