mod utils;
mod types;

use std::process::exit;
use serde::{Deserialize, Serialize};
use utils::files::{load_password, save_password};
use utils::password::{decrypt_password, encrypt_password};
use types::action::Commands;
use clap::Parser;

#[derive(Parser)]
#[command(name = "Password Manager", version = "1.0", author = "B Mathieu <mathieu.busse24@gmail.com>")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Serialize, Deserialize, Debug)]
struct PasswordEntry {
    id: String,
    password: Vec<u8>,
    description: String,
    nonce: [u8; 12],
}

fn main() {
    let cli = Cli::parse();
    let file_path: &str = "passwords.json";
    let key = [0u8; 32];

    let mut passwords= match load_password(&file_path) {
        Ok(list) => list,
        Err(e) => {
            println!("Error : {}", e);
            exit(1);
        }
    };

    match cli.command {
        Commands::Add { id, password, description } => {
            let password = encrypt_password(&password, &key);
            let entry = PasswordEntry {
                id,
                password: password.0,
                description,
                nonce: password.1,
            };
            passwords.push(entry);
            match save_password(&passwords, &file_path) {
                Ok(_) => println!("Password added"),
                Err(e) =>  println!("Error : {}", e)
            }
        },
        Commands::List => {
            if passwords.is_empty() {
                println!("No passwords");
                exit(0);
            }
            for password in passwords {
                println!("{} - {}", password.id, password.description);
            }
        },
        Commands::Search { id } => {
            if let Some(password) = passwords.iter().find(|p| p.id == id){
                let pass = decrypt_password(&password.password, &key, &password.nonce);
                println!("{} - {} - Password: {}", password.id, password.description, pass);
            }else {
                println!("Password not found");
            }
        },
        Commands::Delete { id } => {
            if let Some(_) = passwords.iter().find(|p| p.id == id){
                passwords.retain(|p| p.id != id);
                match save_password(&passwords, &file_path) {
                    Ok(_) => println!("Password deleted"),
                    Err(e) =>  println!("Error : {}", e)
                }
            }
        }
    }


}

