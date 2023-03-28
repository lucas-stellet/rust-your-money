use std::io;

pub type CommandResult = Result<Command, CommandError>;

#[derive(Debug, PartialEq)]
pub enum Command {
    Deposit(String, u64),
    Withdraw(String, u64),
    Send(String, String, u64),
    Quit,
    Print,
    Help,
}

#[derive(Debug)]
pub enum CommandError {
    WrongNumberOfArguments,
    UnknownArgument(String),
    WrongArgumentType(String),
}

impl Command {
    pub fn parse_args(args: String) -> CommandResult {
        let args: Vec<&str> = args.split(' ').collect();
        let command = args[0];

        match command {
            "deposit" => Self::parse_deposit(args),
            "withdraw" => Self::parse_withdraw(args),
            "send" => Self::parse_send(args),
            "print" => Ok(Command::Print),
            "quit" => Ok(Command::Quit),
            "help" => Ok(Command::Help),
            _ => Err(CommandError::UnknownArgument(command.to_string())),
        }
    }

    fn parse_deposit(args: Vec<&str>) -> CommandResult {
        let args = args
            .get(1..=2)
            .ok_or(CommandError::WrongNumberOfArguments)?;

        let account_name = Self::is_account_name(args[0])?;

        let amount = args[1].parse::<u64>().map_err(|_| {
            CommandError::WrongArgumentType(String::from("expected non-negative integer as amount"))
        })?;

        Ok(Command::Deposit(String::from(account_name), amount))
    }

    fn parse_withdraw(args: Vec<&str>) -> CommandResult {
        let args = args
            .get(1..=2)
            .ok_or(CommandError::WrongNumberOfArguments)?;

        let account_name = Self::is_account_name(args[0])?;

        let amount = args[1].parse::<u64>().map_err(|_| {
            CommandError::WrongArgumentType(String::from("expected non-negative integer as amount"))
        })?;

        Ok(Command::Withdraw(String::from(account_name), amount))
    }

    fn parse_send(args: Vec<&str>) -> CommandResult {
        let args = args
            .get(1..=3)
            .ok_or(CommandError::WrongNumberOfArguments)?;

        let sender_account_name = Self::is_account_name(args[0])?;
        let receiver_account_name = Self::is_account_name(args[1])?;

        let amount = args[2].parse::<u64>().map_err(|_| {
            CommandError::WrongArgumentType(String::from("expected non-negative integer as amount"))
        })?;

        Ok(Command::Send(
            String::from(sender_account_name),
            String::from(receiver_account_name),
            amount,
        ))
    }

    fn is_account_name(word: &str) -> Result<&str, CommandError> {
        let mut r: Vec<bool> = vec![];

        for w in word.chars() {
            if w.is_alphabetic() {
                r.push(true)
            } else {
                r.push(false)
            }
        }

        if r.contains(&false) {
            Err(CommandError::WrongArgumentType(String::from(
                "expected a name as an account",
            )))
        } else {
            Ok(word)
        }
    }
}

pub fn read_from_stdin(label: &str) -> String {
    let mut input = String::new();

    println!(
        "
        \r{label} 🦀💰\n 
        \rEnter the command 'help' to get more information
    "
    );

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}
