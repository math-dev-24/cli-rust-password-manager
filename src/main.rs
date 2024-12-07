mod utils;
mod types;

use std::process::exit;
use utils::files::{load_password, save_password};
use types::action::{Commands};
use clap::Parser;
use crate::types::password::PasswordEntry;

#[derive(Parser)]
#[command(name = "Password Manager", version = "1.0", author = "B Mathieu <mathieu.busse24@gmail.com>")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
            if passwords.iter().any(|p| p.id == id) {
                println!("Password already exists");
                exit(1);
            }
            let entry = PasswordEntry::new(id, password, description, &key);
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
                let pass: String = password.get_password(&key);
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

