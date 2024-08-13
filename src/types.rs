use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SPVMError {
    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),

    #[error("Invalid transaction")]
    InvalidTransaction,

    #[error("Transaction hash mismatch")]
    TransactionHashMismatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TxType {
    Mint = 0,
    Transfer = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionContent {
    pub from: String,
    pub tx_type: TxType,
    pub tx_param: Vec<u8>,
    pub nonce: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintTransactionParams {
    pub token_ticker: String,
    pub owner: String,
    pub supply: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferTransactionParams {
    pub token_ticker: String,
    pub to: String,
    pub amount: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPVMTransaction {
    pub tx_content: TransactionContent,
    pub transaction_hash: Vec<u8>,
    #[allow(dead_code)]
    pub signature: Vec<u8>,
}
