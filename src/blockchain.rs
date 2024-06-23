use chrono::prelude::*;
use serde_derive::Serialize;
use sha2::{Digest, Sha256};
use std::fmt::Write;

/// Represents a transaction in the blockchain.
#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

/// Represents the header of a block in the blockchain.
#[derive(Debug, Serialize)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

/// Represents a block in the blockchain.
#[derive(Serialize, Debug)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

/// Represents the blockchain.
pub struct Chain {
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    /// Creates a new blockchain with the specified miner address and difficulty level.
    ///
    /// # Arguments
    ///
    /// * `miner_addr` - A `String` that holds the address of the miner.
    /// * `difficulty` - A `u32` that sets the mining difficulty.
    ///
    /// # Returns
    ///
    /// * `Chain` - A new instance of the `Chain` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// let miner_address = String::from("miner1");
    /// let difficulty = 4;
    /// let blockchain = Chain::new(miner_address, difficulty);
    /// ```
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

    /// Creates a new transaction and adds it to the current list of transactions.
    ///
    /// # Arguments
    ///
    /// * `sender` - A `String` that holds the address of the sender.
    /// * `receiver` - A `String` that holds the address of the receiver.
    /// * `amount` - An `f32` that specifies the amount of currency to be transferred.
    ///
    /// # Returns
    ///
    /// * `bool` - Always returns `true` indicating the transaction was added successfully.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut blockchain = Chain::new(String::from("miner1"), 4);
    /// let result = blockchain.new_transaction(String::from("sender1"), String::from("receiver1"), 50.0);
    /// assert!(result);
    /// ```
    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.curr_trans.push(Transaction {
            sender,
            receiver,
            amount,
        });

        true
    }

    /// Returns the hash of the most recent block in the chain.
    ///
    /// If the chain is empty, it returns a default hash composed of 64 ASCII '0' characters.
    ///
    /// # Returns
    ///
    /// * `String` - The hash of the most recent block or a default hash if the chain is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let blockchain = Chain::new(String::from("miner1"), 4);
    /// let hash = blockchain.last_hash();
    /// println!("Last block hash: {}", hash);
    /// ```
    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };

        Chain::hash(&block.header)
    }

    /// Updates the difficulty level of the blockchain.
    ///
    /// # Arguments
    ///
    /// * `difficulty` - A `u32` that specifies the new difficulty level.
    ///
    /// # Returns
    ///
    /// * `bool` - Always returns `true` indicating the difficulty was updated successfully.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut blockchain = Chain::new(String::from("miner1"), 4);
    /// let result = blockchain.update_difficulty(5);
    /// assert!(result);
    /// assert_eq!(blockchain.difficulty, 5);
    /// ```
    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    /// Updates the reward for mining a new block.
    ///
    /// # Arguments
    ///
    /// * `reward` - An `f32` that specifies the new reward amount.
    ///
    /// # Returns
    ///
    /// * `bool` - Always returns `true` indicating the reward was updated successfully.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut blockchain = Chain::new(String::from("miner1"), 4);
    /// let result = blockchain.update_reward(50.0);
    /// assert!(result);
    /// assert_eq!(blockchain.reward, 50.0);
    /// ```
    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    /// Generates a new block, adds a reward transaction, and appends it to the blockchain.
    ///
    /// # Returns
    ///
    /// * `bool` - Always returns `true` indicating the block was generated and added successfully.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut blockchain = Chain::new(String::from("miner1"), 4);
    /// let result = blockchain.generate_new_block();
    /// assert!(result);
    /// assert_eq!(blockchain.chain.len(), 2); // Initial block + new block
    /// ```
    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: Utc::now().timestamp_millis(),
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        let reward_trans = Transaction {
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

        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    /// Generates the Merkle root hash for a list of transactions.
    ///
    /// # Arguments
    ///
    /// * `curr_trans` - A `Vec<Transaction>` that contains the current list of transactions.
    ///
    /// # Returns
    ///
    /// * `String` - The Merkle root hash as a hexadecimal string.
    ///
    /// # Examples
    ///
    /// ```
    /// let transactions = vec![
    ///     Transaction { sender: String::from("Alice"), receiver: String::from("Bob"), amount: 10.0 },
    ///     Transaction { sender: String::from("Bob"), receiver: String::from("Charlie"), amount: 5.0 },
    /// ];
    /// let merkle_root = Chain::get_merkle(transactions);
    /// println!("Merkle root: {}", merkle_root);
    /// ```
    fn get_merkle(curr_trans: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &curr_trans {
            let hash = Chain::hash(t);

            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let h2 = merkle.remove(0);
            h1.push_str(&h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }
        merkle.pop().unwrap()
    }

    /// Performs proof-of-work to find a valid hash for the block header.
    ///
    /// The function iteratively increments the nonce and computes the hash of the header
    /// until the hash meets the difficulty target (starts with a specific number of leading zeros).
    ///
    /// # Arguments
    ///
    /// * `header` - A mutable reference to the `Blockheader` that needs proof-of-work.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut header = Blockheader {
    ///     timestamp: Utc::now().timestamp_millis(),
    ///     nonce: 0,
    ///     pre_hash: String::new(),
    ///     merkle: String::new(),
    ///     difficulty: 4,
    /// };
    /// Chain::proof_of_work(&mut header);
    /// println!("Nonce: {}, Hash: {}", header.nonce, Chain::hash(&header));
    /// ```
    pub fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];

            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                }
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            };
        }
    }

    /// Computes the SHA-256 hash of a serializable item.
    ///
    /// # Arguments
    ///
    /// * `item` - A reference to an item that implements the `serde::Serialize` trait.
    ///
    /// # Returns
    ///
    /// * `String` - The SHA-256 hash of the item as a hexadecimal string.
    ///
    /// # Examples
    ///
    /// ```
    /// #[derive(Serialize)]
    /// struct Example {
    ///     data: String,
    /// }
    ///
    /// let item = Example {
    ///     data: String::from("hello"),
    /// };
    /// let hash = Chain::hash(&item);
    /// println!("Hash: {}", hash);
    /// ```
    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let res = hasher.finalize();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    /// Converts a slice of bytes into a hexadecimal string representation.
    ///
    /// # Arguments
    ///
    /// * `vec_res` - A slice of `u8` bytes to convert into hexadecimal.
    ///
    /// # Returns
    ///
    /// * `String` - The hexadecimal string representation of the input bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// let bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // Equivalent to "Hello" in ASCII
    /// let hex_string = Chain::hex_to_string(&bytes);
    /// println!("Hexadecimal string: {}", hex_string);
    /// ```
    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("unable to write");
        }

        s
    }
}
