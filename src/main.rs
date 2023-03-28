#![allow(clippy::enum_variant_names)]

mod banking;
mod cli;

use banking::account::*;
use cli::interface::*;

const APPLICATION_LABEL: &str = "RustYourMoney";

fn main() {
    let mut ledger = Accounts::new();

    loop {
        let input = read_from_stdin(APPLICATION_LABEL);

        let command = Command::parse_args(input);

        match command {
            Ok(command) => {
                if command == Command::Quit {
                    break;
                }

                if command == Command::Help {
                    println!(
                        "\n\r❓ Help ❓
                        
                \rDigit one of commands below:
        
                \r=> deposit <account: str> <amount: int>
                \r=> withdraw <account: str> <amount: int>
                \r=> send <sender: str> <recipient: str> <amount: int>
                    "
                    );
                }
                match command {
                    Command::Deposit(account, amount) => {
                        let _tx = ledger.deposit(&account, amount).unwrap();
                        println!("\n✅ Deposit done successfully");
                    }
                    Command::Withdraw(account, amount) => {
                        let _tx = ledger.withdraw(&account, amount).unwrap();
                        println!("\n✅ Withdraw done successfully");
                    }
                    Command::Send(sender, receiver, amount) => {
                        let _tx = ledger.send(&sender, &receiver, amount).unwrap();
                        println!("\n✅ Money sent successfully,");
                    }
                    Command::Print => println!("\nCurrent ledger: \n{}", ledger.print()),
                    _ => {}
                }
            }
            Err(command_error) => {
                match command_error {
                    CommandError::UnknownArgument(argument) => {
                        eprintln!("\n🚨 Error: unknown argument '{argument}'")
                    }
                    CommandError::WrongNumberOfArguments => {
                        eprintln!("\n🚨 Error: the number of arguments is not correct for the given command")
                    }
                    CommandError::WrongArgumentType(argument) => {
                        eprintln!("\n🚨 Error: {argument}")
                    }
                }
            }
        }
    }
}
