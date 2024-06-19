use chrono::prelude::*;
use serde;
use serde_derive::Serialize;
use serde_json;
use sha2::{Digest, Sha256};
use std::fmt::Write;

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
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };
        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.curr_trans.push(Transition {
            sender,
            receiver,
            amount,
        });

        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };

        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: Utc::now().timestamp_millis(),
            nonce: 0,
            pre_hash: self.last_hash(),
            difficulty: self.difficulty,
        };

        let reward_trans = Transition {
            sender: String::from("Root"),
            receiver: self.miner_addr.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![],
        };

        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.curr_trans);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());

        Chain::proof_of_work(&mut block.header);

        println!("{:?}", &block);
        self.chain.push(block);
        true
    }
}
