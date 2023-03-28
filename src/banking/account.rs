use std::collections::HashMap;

use super::transaction;

#[derive(Debug)]
pub enum AccountingError {
    // Add variants here for account not found, account underfunded and account overfunded
    AccountNotFound(String),
    AccountUnderFunded(String, u64),
    AccountOverFunded(String, u64),
}

#[derive(Debug)]
pub struct Accounts {
    accounts: HashMap<String, u64>,
}

impl Accounts {
    pub fn new() -> Self {
        Accounts {
            accounts: Default::default(),
        }
    }

    pub fn deposit(
        &mut self,
        signer: &str,
        amount: u64,
    ) -> Result<transaction::Tx, AccountingError> {
        if let Some(account) = self.accounts.get_mut(signer) {
            (*account)
                .checked_add(amount)
                .map(|r| {
                    *account = r;
                    Some(r)
                })
                .ok_or(AccountingError::AccountOverFunded(
                    signer.to_string(),
                    amount,
                ))
                .map(|_| transaction::Tx::Deposit {
                    account: signer.to_string(),
                    amount,
                })
        } else {
            self.accounts.insert(signer.to_string(), amount);
            Ok(transaction::Tx::Deposit {
                account: signer.to_string(),
                amount,
            })
        }
    }

    pub fn withdraw(
        &mut self,
        signer: &str,
        amount: u64,
    ) -> Result<transaction::Tx, AccountingError> {
        if let Some(signer_account) = self.accounts.get_mut(signer) {
            (*signer_account)
                .checked_sub(amount)
                .map(|r| {
                    *signer_account = r;
                    transaction::Tx::Withdraw {
                        account: signer.to_string(),
                        amount,
                    }
                })
                .ok_or(AccountingError::AccountUnderFunded(
                    signer.to_string(),
                    amount,
                ))
        } else {
            Err(AccountingError::AccountNotFound(signer.to_string()))
        }
    }

    pub fn send(
        &mut self,
        sender: &str,
        recipient: &str,
        amount: u64,
    ) -> Result<(transaction::Tx, transaction::Tx), AccountingError> {
        if let (Some(_sender_account), Some(_recipient_account)) =
            (self.accounts.get(sender), self.accounts.get(recipient))
        {
            let sender_tx = self.withdraw(sender, amount)?;

            let recipient_tx = self.deposit(recipient, amount)?;

            return Ok((sender_tx, recipient_tx));
        }

        Err(AccountingError::AccountOverFunded(
            sender.to_string(),
            amount,
        ))
    }

    pub fn print(&self) -> String {
        let mut output = String::new();

        for (name, amount) in &self.accounts {
            let account = format!(
                "\n\rAccount:
            \r   name: {}
            \r   amount: {}
            ",
                &name, amount
            );

            output.push_str(&account[..])
        }

        output
    }
}
