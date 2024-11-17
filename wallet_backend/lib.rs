use candid::{CandidType, Principal};
use serde::Deserialize;
use ic_cdk::api::caller;
use ic_cdk_macros::{init, query, update};
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(CandidType, Deserialize, Clone)]
struct Transaction {
    from: Principal,
    to: Principal,
    amount: u64,
}

#[derive(CandidType, Deserialize)]
struct TokenWallet {
    balances: HashMap<Principal, u64>,
    owner: Principal,
    transactions: Vec<Transaction>,
}

lazy_static! {
    static ref WALLET: Mutex<TokenWallet> = Mutex::new(TokenWallet {
        balances: HashMap::new(),
        owner: Principal::anonymous(),
        transactions: Vec::new(),
    });
}

#[init]
fn init() {
    let caller = caller();
    let mut wallet = WALLET.lock().expect("Failed to acquire the lock on WALLET");
    if wallet.owner != Principal::anonymous() {
        ic_cdk::trap("Canister already initialized!");
    }
    wallet.owner = caller;
    wallet.balances.insert(caller, 1000);
    ic_cdk::println!("Initial balance for {:?}: {}", caller, 1000);
}


#[update]
fn transfer(to: Principal, amount: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();
    let mut wallet = WALLET.lock().expect("Failed to acquire the lock on WALLET");

    ic_cdk::println!("Caller: {:?}", caller);
    ic_cdk::println!("Recipient: {:?}", to);

    let sender_balance = wallet.balances.get(&caller).cloned().unwrap_or_default();
    ic_cdk::println!("Sender balance before transfer: {}", sender_balance);

    if sender_balance < amount {
        return Err("Insufficient balance".to_string());
    }

    // Update sender and receiver balances
    let receiver_balance = wallet.balances.entry(to).or_insert(0);
    *receiver_balance += amount;
    wallet.balances.insert(caller, sender_balance - amount);

    // Log the updated balances
    ic_cdk::println!("Sender balance after transfer: {}", wallet.balances.get(&caller).cloned().unwrap_or_default());
    ic_cdk::println!("Receiver balance after transfer: {}", wallet.balances.get(&to).cloned().unwrap_or_default());

    // Add transaction to log
    wallet.transactions.push(Transaction {
        from: caller,
        to,
        amount,
    });

    Ok(())
}

#[query]
fn get_balance(address: Principal) -> u64 {
    let wallet = WALLET.lock().expect("Failed to acquire the lock on WALLET");
    *wallet.balances.get(&address).unwrap_or(&0)
}

#[query]
fn get_transactions() -> Vec<Transaction> {
    let transactions = WALLET.lock().expect("Failed to acquire the lock on WALLET");
    transactions.transactions.clone() // Clone the transactions list
}

#[query]
fn get_all_balances() -> HashMap<Principal, u64> {
    let wallet = WALLET.lock().expect("Failed to acquire the lock on WALLET");
    wallet.balances.clone() // Return a clone of all balances
}


