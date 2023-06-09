use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use cqrs_es::{DomainEvent, Aggregate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub enum BankAccountCommand {
    OpenAccount { account_id: String },
    DepositMoney { amount: f64 },
    WithdrawMoney { amount: f64 },
    WriteCheck { check_number: String, amount: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BankAccountEvent {
    AccountOpened {
        account_id: String,
    },
    CustomerDepositedMoney {
        amount: f64,
        balance: f64,
    },
    CustomerWithdrewCash {
        amount: f64,
        balance: f64,
    },
    CustomerWroteCheck {
        check_number: String,
        amount: f64,
        balance: f64,
    },
}

impl DomainEvent for BankAccountEvent {
    fn event_type(&self) -> String {
        let event_type: &str = match self {
            BankAccountEvent::AccountOpened { .. } => "AccountOpened",
            BankAccountEvent::CustomerDepositedMoney { .. } => "CustomerDepositedMoney",
            BankAccountEvent::CustomerWithdrewCash { .. } => "CustomerWithdrewCash",
            BankAccountEvent::CustomerWroteCheck { .. } => "CustomerWroteCheck",
        };
        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug)]
pub struct BankAccountError(String);

impl Display for BankAccountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for BankAccountError {}

impl From<&str> for BankAccountError {
    fn from(message: &str) -> Self {
        BankAccountError(message.to_string())
    }
}

pub struct BankAccountServices;

impl BankAccountServices {
    async fn atm_withdrawal(&self, atm_id: &str, amount: f64) -> Result<(), AtmError> {
        Ok(())
    }

    async fn validate_check(&self, account: &str, check: &str) -> Result<(), CheckingError> {
        Ok(())
    }
}
pub struct AtmError;
pub struct CheckingError;

#[derive(Serialize, Default, Deserialize)]
pub struct BankAccount {
    opened: bool,
    // this is a floating point for our example, don't do this IRL
    balance: f64,
}

#[async_trait]
impl Aggregate for BankAccount {
    type Command = BankAccountCommand;
    type Event = BankAccountEvent;
    type Error = BankAccountError;
    type Services = BankAccountServices;

    // This identifier should be unique to the system.
    fn aggregate_type() -> String {
        "Account".to_string()
    }

    // The aggregate logic goes here. Note that this will be the _bulk_ of a CQRS system
    // so expect to use helper functions elsewhere to keep the code clean.
    async fn handle(
        &self,
        command: Self::Command,
        services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        todo!()
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            BankAccountEvent::AccountOpened { .. } => self.opened = true,

            BankAccountEvent::CustomerDepositedMoney { amount: _, balance } => {
                self.balance = balance;
            }

            BankAccountEvent::CustomerWithdrewCash { amount: _, balance } => {
                self.balance = balance;
            }

            BankAccountEvent::CustomerWroteCheck {
                check_number: _,
                amount: _,
                balance,
            } => {
                self.balance = balance;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
