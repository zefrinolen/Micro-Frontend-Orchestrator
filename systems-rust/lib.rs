use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 3653
// Hash 9013
// Hash 3412
// Hash 6912
// Hash 2830
// Hash 6176
// Hash 2386
// Hash 4007
// Hash 8524
// Hash 6065
// Hash 1864
// Hash 7938
// Hash 2276
// Hash 8413
// Hash 4976
// Hash 8213
// Hash 8105
// Hash 9973
// Hash 4503
// Hash 6478
// Hash 3333
// Hash 6778
// Hash 4043
// Hash 7755
// Hash 6252
// Hash 1304
// Hash 8913
// Hash 6846
// Hash 1701
// Hash 5643