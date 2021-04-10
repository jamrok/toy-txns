use crate::precision_64;
use crate::transactions::{TransactionState, TransactionStatus};

use std::{
    collections::{BTreeMap, HashMap},
    fmt,
};

pub struct ClientDB {
    table: BTreeMap<u16, Client>,
}

impl ClientDB {
    pub fn new() -> ClientDB {
        ClientDB {
            table: BTreeMap::new(),
        }
    }
    pub fn fetch(&mut self, id: u16) -> Option<&mut Client> {
        // Create a client if they're missing
        if self.table.get(&id).is_none() {
            self.table.insert(id, Client::new(id));
        }

        // Get a mutable reference to a client
        match self.table.get_mut(&id) {
            Some(c) => {
                // Debugging
                // println!("  Fetched {}", c.info());
                Some(c)
            }
            None => None,
        }
    }

    /// Get the formatted output for the ClientDB
    pub fn dump(&self) -> String {
        let mut output = String::new();
        output.push_str(
            format!(
                "{},{},{},{},{}",
                "client", "available", "held", "total", "locked"
            )
            .as_str(),
        );
        for line in self.table.iter() {
            output.push_str(format!("\n{}", line.1).as_str());
        }
        output
    }

    pub fn records(&self) -> usize {
        self.table.len()
    }
}

struct ClientTransactions {
    amount: f64,
    state: TransactionState,
}

impl ClientTransactions {
    pub fn new(amount: f64, state: TransactionState) -> ClientTransactions {
        ClientTransactions { amount, state }
    }
}

pub struct Client {
    id: u16,
    pub account: Account,
    transactions: HashMap<u32, ClientTransactions>,
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{}",
            self.id,
            self.account.available(),
            self.account.held(),
            self.account.total(),
            self.account.locked,
        )
    }
}

impl Client {
    pub fn new<'a>(id: u16) -> Client {
        Client {
            id,
            account: Account::new(),
            transactions: HashMap::new(),
        }
    }
    // Debugging
    pub fn info(&self) -> String {
        format!(
            "{} | Avail: {} | Held: {} | Total: {} | Locked: {}",
            self.id,
            self.account.available,
            self.account.held,
            self.account.total,
            self.account.locked,
        )
    }
    pub fn deposit(&mut self, amount: f64, txn_id: u32) -> TransactionStatus {
        self.transactions.insert(
            txn_id,
            ClientTransactions::new(amount, TransactionState::Cleared),
        );
        self.account.available += amount;
        self.account.total += amount;
        TransactionStatus::Success
    }

    pub fn withdraw(&mut self, amount: f64, txn_id: u32) -> TransactionStatus {
        if self.account.available >= amount {
            self.account.available -= amount;
            self.account.total -= amount;
            self.transactions.insert(
                txn_id,
                ClientTransactions::new(amount, TransactionState::Cleared),
            );
            TransactionStatus::Success
        } else {
            TransactionStatus::InsufficientFunds
        }
    }

    pub fn dispute(&mut self, txn_id: u32) -> TransactionStatus {
        let transaction = match self.transactions.get_mut(&txn_id) {
            Some(v) => v,
            None => return TransactionStatus::NoTransactionToDispute,
        };

        // Only deduct from available if there is enough funds to cover the dispute
        if self.account.available >= transaction.amount {
            self.account.available -= transaction.amount;
            self.account.held += transaction.amount;
            transaction.state = TransactionState::Disputed;
            TransactionStatus::Success
        } else {
            //If there is not enough funds to handle the dispute we should lock the account
            self.account.locked = true;
            transaction.state = TransactionState::AttemptedDispute;
            TransactionStatus::InsufficientFundsForDispute
        }
    }

    pub fn resolve(&mut self, txn_id: u32) -> TransactionStatus {
        let transaction = match self
            .transactions
            .get_mut(&txn_id)
            .filter(|t| t.state == TransactionState::Disputed)
        {
            Some(v) => v,
            None => return TransactionStatus::NoDisputeToResolve,
        };

        self.account.available += transaction.amount;
        self.account.held -= transaction.amount;
        transaction.state = TransactionState::Resolved;
        TransactionStatus::Success
    }

    pub fn chargeback(&mut self, txn_id: u32) -> TransactionStatus {
        let transaction = match self
            .transactions
            .get_mut(&txn_id)
            .filter(|t| t.state == TransactionState::Disputed)
        {
            Some(v) => v,
            None => return TransactionStatus::NoDisputeToChargeBack,
        };

        self.account.total -= transaction.amount;
        self.account.held -= transaction.amount;
        self.account.locked = true;
        transaction.state = TransactionState::ChargedBack;
        TransactionStatus::Success
    }
}
pub struct Account {
    // state: Option<Box<dyn State>>,
    available: f64,
    held: f64,
    total: f64,
    locked: bool,
}

impl Account {
    // Return a new active account
    pub fn new() -> Account {
        Account {
            // state: Some(Box::new(Unfunded {})),
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
    pub fn available(&self) -> f64 {
        precision_64(self.available, 4)
    }
    pub fn held(&self) -> f64 {
        precision_64(self.held, 4)
    }
    pub fn total(&self) -> f64 {
        precision_64(self.total, 4)
    }

    pub fn locked(&self) -> bool {
        self.locked
    }
}

/*
TODO:
- Use the state pattern on the Account struct as well
- Move the account between states based on transactions
- Locked accounts should inherently not be able to do any transactions (based on AccountState instead of account.locked check)

trait AccountState {}

/// Base state when new client is created.
struct Unfunded {}
impl AccountState for Unfunded {}

/// Promoted from Unfunded after the first Deposit
struct Active {}
impl AccountState for Active {}

/// Active accounts get changed to Locked due to ChargeBacks or other activities that warrant a lock.
/// New transactions will not be processed until the account is reactivated.
struct Locked {}
impl AccountState for Locked {}

*/
