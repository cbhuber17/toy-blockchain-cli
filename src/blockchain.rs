use serde;
use serde_derive::Serialize;
use serde_json;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use time;

#[derive(Debug, Clone, Serialize)]
struct Transition {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Serialize, Debug)]
struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transition>,
}

pub struct Chain {
    chain: Vec<Block>,
    curr_trans: Vec<Transition>,
    difficulty: u32,
    miner_addr: String,
    reward: u32,
}
