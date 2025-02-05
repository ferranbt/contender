use alloy::primitives::{Address, TxHash};
use serde::Serialize;

use crate::Result;

#[derive(Debug, Serialize, Clone)]
pub struct RunTx {
    pub tx_hash: TxHash,
    #[serde(rename = "start_time")]
    pub start_timestamp: usize,
    #[serde(rename = "end_time")]
    pub end_timestamp: usize,
    pub block_number: u64,
    pub gas_used: u128,
}

pub trait DbOps {
    fn create_tables(&self) -> Result<()>;

    /// Insert a new run into the database. Returns run_id.
    fn insert_run(&self, timestamp: u64, tx_count: usize) -> Result<u64>;

    fn num_runs(&self) -> Result<u64>;

    fn insert_named_tx(
        &self,
        name: String,
        tx_hash: TxHash,
        contract_address: Option<Address>,
    ) -> Result<()>;

    fn insert_named_txs(&self, named_txs: Vec<(String, TxHash, Option<Address>)>) -> Result<()>;

    fn get_named_tx(&self, name: &str) -> Result<(TxHash, Option<Address>)>;

    fn insert_run_tx(&self, run_id: u64, tx: RunTx) -> Result<()>;

    fn insert_run_txs(&self, run_id: u64, run_txs: Vec<RunTx>) -> Result<()>;

    fn get_run_txs(&self, run_id: u64) -> Result<Vec<RunTx>>;
}
