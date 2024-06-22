use std::{
    io::{self, Write},
    process,
};

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    get_input("miner address: ", &mut miner_addr);
    get_input("difficulty (1 or 2): ", &mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("Integer required");

    if diff > 2 {
        println!("Error: Difficulty too high, please select 1 or 2.");
        process::exit(1);
    }

    println!("Generating genesis block! Please wait.\n");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("\nMenu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit\n");
        print!("Enter your choice: ");
        let _ = io::stdout().flush();
        choice.clear();
        let _ = io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting!");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                get_input("sender address: ", &mut sender);
                get_input("receiver address: ", &mut receiver);
                get_input("amount: ", &mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("Transaction added!"),
                    false => println!("Transaction failed!"),
                }
            }
            2 => {
                println!("Generating block, please wait!");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully!"),
                    false => println!("Block generation failed!"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                get_input("new difficulty: ", &mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());

                match res {
                    true => println!("Updated difficulty!"),
                    false => println!("Failed updated difficulty!"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                get_input("new reward: ", &mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse().unwrap());

                match res {
                    true => println!("Updated reward!"),
                    false => println!("Failed to update reward!"),
                }
            }
            _ => {
                println!("\tInvalid option selected, please retry\t");
            }
        }
    }
}

fn get_input(input_str: &str, var: &mut String) {
    print!("Enter a {}", input_str);
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(var);
}
