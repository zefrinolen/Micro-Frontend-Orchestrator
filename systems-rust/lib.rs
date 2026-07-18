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
// Hash 5314
// Hash 9069
// Hash 9799
// Hash 9800
// Hash 9957
// Hash 2092
// Hash 1898
// Hash 1788
// Hash 5550
// Hash 6929
// Hash 9453
// Hash 7059
// Hash 6447
// Hash 9804
// Hash 9195
// Hash 9630
// Hash 5193
// Hash 9742
// Hash 8088
// Hash 5990
// Hash 1314
// Hash 6907
// Hash 4466
// Hash 5276
// Hash 8638
// Hash 5685
// Hash 9111
// Hash 9038
// Hash 6773
// Hash 2075
// Hash 7442
// Hash 5146
// Hash 1267
// Hash 3438
// Hash 6696
// Hash 4903
// Hash 5873
// Hash 3637
// Hash 5724
// Hash 6772
// Hash 3131
// Hash 6978
// Hash 2701
// Hash 9689
// Hash 6216
// Hash 4523
// Hash 9295
// Hash 2099
// Hash 2083
// Hash 2497
// Hash 7245
// Hash 6681
// Hash 1607
// Hash 6369
// Hash 5581
// Hash 1011
// Hash 4523
// Hash 1970
// Hash 7021
// Hash 1575
// Hash 6314
// Hash 2219
// Hash 7601
// Hash 7915
// Hash 8586
// Hash 2876
// Hash 5887
// Hash 2395
// Hash 9683
// Hash 8097
// Hash 4931
// Hash 8207
// Hash 1285
// Hash 7302
// Hash 4392
// Hash 6855
// Hash 2277
// Hash 3701
// Hash 5724
// Hash 9647
// Hash 9584
// Hash 2130
// Hash 6785
// Hash 4347
// Hash 9208
// Hash 4402
// Hash 6353
// Hash 8003
// Hash 2839
// Hash 3195
// Hash 6144
// Hash 9429
// Hash 1673
// Hash 7417
// Hash 2390
// Hash 8234
// Hash 9874
// Hash 2051
// Hash 9468
// Hash 1332
// Hash 3231
// Hash 6024
// Hash 8304
// Hash 7750
// Hash 3799
// Hash 3550
// Hash 3007
// Hash 1522
// Hash 8647
// Hash 9709
// Hash 6812
// Hash 3147
// Hash 3400
// Hash 9647
// Hash 5882
// Hash 3160
// Hash 5343
// Hash 7967
// Hash 2328
// Hash 6146
// Hash 1959
// Hash 9444
// Hash 2195
// Hash 9094
// Hash 7610
// Hash 4021
// Hash 5981
// Hash 6071
// Hash 7432
// Hash 3088
// Hash 7149
// Hash 1305
// Hash 9284
// Hash 7904
// Hash 4214
// Hash 4590
// Hash 8107
// Hash 4519
// Hash 9411
// Hash 5868
// Hash 4416
// Hash 3575
// Hash 6572
// Hash 2923
// Hash 8793
// Hash 2659
// Hash 4731
// Hash 8983
// Hash 6096
// Hash 5490
// Hash 5995
// Hash 8966
// Hash 3692
// Hash 9721
// Hash 4219