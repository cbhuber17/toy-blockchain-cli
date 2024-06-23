# Blockchain CLI in Rust

This is a simple command-line interface (CLI) application for a blockchain implemented in Rust. The application allows you to manage transactions, mine blocks, and adjust blockchain parameters such as difficulty and mining reward.

## Features

- **New Transaction:** Create and add transactions to the blockchain.
- **Mine Block:** Generate new blocks by mining them.
- **Change Difficulty:** Adjust the mining difficulty level.
- **Change Reward:** Modify the mining reward for successfully mined blocks.
- **Genesis Block:** Automatically generates a genesis block upon initialization.

## Prerequisites

- Rust programming language and Cargo should be installed. [Install Rust](https://www.rust-lang.org/tools/install).

## Installation

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd blockchain-cli-rust

   ```

2. Build the project:

```
cargo build
```

## Usage

Run the application:

```
cargo run
```

## Menu Options

Upon running the application, you will be presented with a menu to interact with the blockchain:

- New Transaction: Add a new transaction to the blockchain.
- Mine Block: Mine a new block containing pending transactions.
- Change Difficulty: Adjust the difficulty level of mining.
- Change Reward: Update the mining reward for successfully mined blocks.
- Exit: Terminate the application.

Follow the prompts to enter necessary information for each menu option.

### Example

Here's an example session demonstrating the usage of the CLI:

```
$ cargo run

Enter a miner address: miner1
Enter difficulty (1 or 2): 1
Generating genesis block! Please wait.

MENU:

1. New Transaction
2. Mine block
3. Change Difficulty
4. Change Reward
5. Exit

Enter your choice: 1
Enter a sender address: user1
Enter a receiver address: user2
Enter amount: 10.0
Transaction added!

Enter your choice: 2
Generating block, please wait!
Block generated successfully!

Enter your choice: 0
Exiting!
```
