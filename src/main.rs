use spvm::{
    types::{MintTransactionParams, SPVMError, SPVMTransaction, TransactionContent, TxType},
    SPVM,
};

fn main() -> Result<(), SPVMError> {
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
    println!("Account1 BTC balance: {}", balance);

    Ok(())
}
