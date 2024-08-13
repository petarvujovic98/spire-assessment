use std::collections::HashMap;

use types::{
    MintTransactionParams, SPVMError, SPVMTransaction, TransactionContent,
    TransferTransactionParams, TxType,
};

#[cfg(test)]
mod tests;
pub mod types;

#[allow(clippy::upper_case_acronyms)]
#[derive(Default)]
pub struct SPVM {
    initialized_tickers: HashMap<String, bool>,
    state: HashMap<String, HashMap<String, u16>>,
    nonces: HashMap<String, u32>,
}

impl SPVM {
    pub fn new() -> Self {
        SPVM::default()
    }

    pub fn set_balance(&mut self, token_ticker: &str, holder_address: &str, balance: u16) {
        self.initialized_tickers
            .insert(token_ticker.to_string(), true);
        self.state
            .entry(token_ticker.to_string())
            .or_default()
            .insert(holder_address.to_string(), balance);
    }

    pub fn get_balance(&self, token_ticker: &str, holder_address: &str) -> u16 {
        self.state
            .get(token_ticker)
            .and_then(|balances| balances.get(holder_address))
            .cloned()
            .unwrap_or(0)
    }

    pub fn check_validity(&self, tx_content: &TransactionContent) -> Result<bool, SPVMError> {
        match tx_content.tx_type {
            TxType::Mint => {
                let mint_params: MintTransactionParams =
                    bincode::deserialize(&tx_content.tx_param)?;
                Ok(!self
                    .initialized_tickers
                    .contains_key(&mint_params.token_ticker))
            }
            TxType::Transfer => {
                let transfer_params: TransferTransactionParams =
                    bincode::deserialize(&tx_content.tx_param)?;
                Ok(self
                    .initialized_tickers
                    .contains_key(&transfer_params.token_ticker)
                    && self.get_balance(&transfer_params.token_ticker, &tx_content.from)
                        >= transfer_params.amount)
            }
        }
    }

    pub fn execute_raw_transaction(&mut self, raw_tx: &[u8]) -> Result<(), SPVMError> {
        let tx_content: TransactionContent = bincode::deserialize(raw_tx)?;

        if !self.check_validity(&tx_content)? {
            return Err(SPVMError::InvalidTransaction);
        }

        match tx_content.tx_type {
            TxType::Mint => {
                let mint_params: MintTransactionParams =
                    bincode::deserialize(&tx_content.tx_param)?;
                self.set_balance(
                    &mint_params.token_ticker,
                    &mint_params.owner,
                    mint_params.supply,
                );
            }
            TxType::Transfer => {
                let transfer_params: TransferTransactionParams =
                    bincode::deserialize(&tx_content.tx_param)?;
                let from_balance =
                    self.get_balance(&transfer_params.token_ticker, &tx_content.from);
                self.set_balance(
                    &transfer_params.token_ticker,
                    &tx_content.from,
                    from_balance - transfer_params.amount,
                );
                let to_balance =
                    self.get_balance(&transfer_params.token_ticker, &transfer_params.to);
                self.set_balance(
                    &transfer_params.token_ticker,
                    &transfer_params.to,
                    to_balance + transfer_params.amount,
                );
            }
        }

        *self.nonces.entry(tx_content.from.clone()).or_insert(0) += 1;
        Ok(())
    }

    pub fn execute_tx(&mut self, transaction: &SPVMTransaction) -> Result<(), SPVMError> {
        let tx_hash = self.hash_transaction_content(&transaction.tx_content)?;
        if tx_hash != transaction.transaction_hash {
            return Err(SPVMError::TransactionHashMismatch);
        }
        // Assume the signature is valid for simplicity
        self.execute_raw_transaction(&bincode::serialize(&transaction.tx_content)?)?;
        Ok(())
    }

    pub fn hash_transaction_content(
        &self,
        tx_content: &TransactionContent,
    ) -> Result<Vec<u8>, SPVMError> {
        Ok(bincode::serialize(tx_content)?)
    }

    #[allow(dead_code)]
    pub fn execute_block_transactions(
        &mut self,
        transactions: &[SPVMTransaction],
    ) -> Result<(), SPVMError> {
        for transaction in transactions {
            self.execute_tx(transaction)?;
        }
        Ok(())
    }
}
