use crate::client::*;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::any::type_name;
use std::error::Error;

#[derive(Debug, std::cmp::PartialEq)]
pub enum TransactionState {
    Cleared,
    Disputed,
    AttemptedDispute,
    Resolved,
    ChargedBack,
    Unknown,
}

#[derive(Debug)]
pub enum TransactionStatus {
    AccountLockedTransactionBlocked,
    InsufficientFunds,
    InsufficientFundsForDispute,
    InvalidTransaction,
    NoDisputeToChargeBack,
    NoDisputeToResolve,
    NoTransactionToDispute,
    Success,
}

#[derive(Deserialize, Debug)]
pub struct TransactionDetails {
    action: String,
    client: u16,
    id: u32,
    pub amount: Option<f64>,
}

impl TransactionDetails {
    /// Transform the transaction into one of the known types
    pub fn register(self) -> Transaction {
        Transaction::new(self)
    }
}
pub struct Transaction {
    details: TransactionDetails,
    state: Option<Box<dyn TransactionType>>,
}

impl Transaction {
    /// Maps the transaction types to the corresponding struct
    fn new(details: TransactionDetails) -> Transaction {
        let txtype: Box<dyn TransactionType> = match details.action.as_str() {
            "deposit" => Box::new(Deposit {}),
            "withdrawal" => Box::new(Withdrawal {}),
            "dispute" => Box::new(Dispute {}),
            "resolve" => Box::new(Resolve {}),
            "chargeback" => Box::new(Chargeback {}),
            _ => Box::new(Unknown {}),
        };
        Transaction {
            details,
            state: Some(txtype),
        }
    }

    // Used for Debugging
    fn _info(&self) -> String {
        self.state.as_ref().unwrap().info(self)
    }

    /// Updates the client according to the rules for the TransactionType
    fn update_client(&self, client: &mut Client) -> TransactionStatus {
        // Block transactions if account is locked
        if client.account.locked() {
            return TransactionStatus::AccountLockedTransactionBlocked;
        }
        self.state.as_ref().unwrap().update_client(self, client)
    }
}

/// Use the State Pattern for each TransactionType so each implementation is
/// responsible for how to process the transaction. This should allow for easier
/// extendability and maintainability.
trait TransactionType {
    /// Must take ownership of Box<Self> and return a new TransactionType state
    fn register(self: Box<Self>) -> Box<dyn TransactionType>;
    // fn update(&self, transaction: &TransactionDetails);

    fn info<'a>(&self, _txn: &'a Transaction) -> String {
        type_name::<Self>().split("::").last().unwrap().to_string()
    }
    fn update_client<'a>(&self, txn: &'a Transaction, client: &mut Client) -> TransactionStatus;
}

struct Deposit {}

impl TransactionType for Deposit {
    fn register(self: Box<Self>) -> Box<dyn TransactionType> {
        self
    }

    fn update_client<'a>(&self, txn: &'a Transaction, client: &mut Client) -> TransactionStatus {
        let amount = txn.details.amount.unwrap_or(0.0);
        client.deposit(amount, txn.details.id)
    }
}

struct Withdrawal {}

impl TransactionType for Withdrawal {
    fn register(self: Box<Self>) -> Box<dyn TransactionType> {
        self
    }

    fn update_client<'a>(&self, txn: &'a Transaction, client: &mut Client) -> TransactionStatus {
        let amount = txn.details.amount.unwrap_or(0.0);
        client.withdraw(amount, txn.details.id)
    }
}

struct Dispute {}

impl TransactionType for Dispute {
    fn register(self: Box<Self>) -> Box<dyn TransactionType> {
        self
    }

    fn update_client<'a>(&self, txn: &'a Transaction, client: &mut Client) -> TransactionStatus {
        client.dispute(txn.details.id)
    }
}

struct Resolve {}

impl TransactionType for Resolve {
    fn register(self: Box<Self>) -> Box<dyn TransactionType> {
        self
    }

    fn update_client<'a>(&self, txn: &'a Transaction, client: &mut Client) -> TransactionStatus {
        client.resolve(txn.details.id)
    }
}

struct Chargeback {}

impl TransactionType for Chargeback {
    fn register(self: Box<Self>) -> Box<dyn TransactionType> {
        self
    }

    fn update_client<'a>(&self, txn: &'a Transaction, client: &mut Client) -> TransactionStatus {
        client.chargeback(txn.details.id)
    }
}

struct Unknown {}

impl TransactionType for Unknown {
    fn register(self: Box<Self>) -> Box<dyn TransactionType> {
        self
    }

    fn update_client<'a>(&self, txn: &'a Transaction, _client: &mut Client) -> TransactionStatus {
        eprintln!("Unexpected Transaction type: {:?}", txn.details);
        TransactionStatus::InvalidTransaction
    }
}

/// Main function that processes transactions.
pub struct TransactionManager {}

impl TransactionManager {
    // fn process(txn: Transaction) {}

    pub fn process_input_file(filename: &String) -> Result<String, Box<dyn Error>> {
        // Note: the CSV Reader is buffered and can be used to handle streams (from file or TCP).
        let lines = ReaderBuilder::new()
            // We should determine whether to set has_headers or not.
            // It defaults to true, but we may want to control this with a variable
            .has_headers(true)
            .trim(csv::Trim::All)
            .from_path(filename);

        // Raise an error for calling function to handle, otherwise continue.
        match lines {
            Ok(_) => (),
            Err(err) => return Err(format!("{}: {}", filename, err).into()),
        }
        let mut lines = lines.unwrap();

        let mut client_db = ClientDB::new();

        for result in lines.records() {
            let line = match result {
                Ok(l) => l,
                Err(e) => {
                    // We should log all unprocessed lines so they can be later assessed
                    eprintln!("Error processing line: {:?}", e);
                    continue;
                }
            };

            let txn: TransactionDetails = line.deserialize(None)?;
            // println!("\n{:?}", txn); // Debugging line

            // Transform the transaction into one of the known types
            let txn: Transaction = txn.register();

            // Get the client for the transaction
            let newclient = &mut Client::new(txn.details.client);
            let client = client_db.fetch(txn.details.client).unwrap_or(newclient);

            let _status = txn.update_client(client);

            // Debugging
            // println!(
            //     "    Client info: {} | {:?} | {:?}",
            //     &client.info(),
            //     txn._info(),
            //     _status
            // );
        }

        if client_db.records() > 0 {
            Ok(client_db.dump())
        } else {
            Err("Nothing Processed".into())
        }
    }
}
