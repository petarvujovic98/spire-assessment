use crate::{
    types::{
        MintTransactionParams, SPVMError, SPVMTransaction, TransactionContent,
        TransferTransactionParams, TxType,
    },
    SPVM,
};

#[test]
fn test_mint_transaction() -> Result<(), SPVMError> {
    let mut spvm = SPVM::new();

    let mint_params = MintTransactionParams {
        token_ticker: "BTC".to_string(),
        owner: "account1".to_string(),
        supply: 100,
    };

    let tx_content = TransactionContent {
        from: "account1".to_string(),
        tx_type: TxType::Mint,
        tx_param: bincode::serialize(&mint_params)?,
        nonce: 0,
    };

    let spvm_tx = SPVMTransaction {
        tx_content: tx_content.clone(),
        transaction_hash: spvm.hash_transaction_content(&tx_content)?,
        signature: vec![], // Signature validation is skipped in this example
    };

    spvm.execute_tx(&spvm_tx)?;

    let balance = spvm.get_balance("BTC", "account1");
    assert_eq!(balance, 100);
    Ok(())
}

#[test]
fn test_transfer_transaction() -> Result<(), SPVMError> {
    let mut spvm = SPVM::new();

    // Mint some BTC first
    let mint_params = MintTransactionParams {
        token_ticker: "BTC".to_string(),
        owner: "account1".to_string(),
        supply: 100,
    };

    let tx_content = TransactionContent {
        from: "account1".to_string(),
        tx_type: TxType::Mint,
        tx_param: bincode::serialize(&mint_params)?,
        nonce: 0,
    };

    let spvm_tx = SPVMTransaction {
        tx_content: tx_content.clone(),
        transaction_hash: spvm.hash_transaction_content(&tx_content)?,
        signature: vec![], // Signature validation is skipped in this example
    };

    spvm.execute_tx(&spvm_tx)?;

    // Now transfer some BTC
    let transfer_params = TransferTransactionParams {
        token_ticker: "BTC".to_string(),
        to: "account2".to_string(),
        amount: 50,
    };

    let tx_content = TransactionContent {
        from: "account1".to_string(),
        tx_type: TxType::Transfer,
        tx_param: bincode::serialize(&transfer_params)?,
        nonce: 1,
    };

    let spvm_tx = SPVMTransaction {
        tx_content: tx_content.clone(),
        transaction_hash: spvm.hash_transaction_content(&tx_content)?,
        signature: vec![], // Signature validation is skipped in this example
    };

    spvm.execute_tx(&spvm_tx)?;

    let balance1 = spvm.get_balance("BTC", "account1");
    let balance2 = spvm.get_balance("BTC", "account2");

    assert_eq!(balance1, 50);
    assert_eq!(balance2, 50);
    Ok(())
}
