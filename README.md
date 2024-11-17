# **ICP Token Wallet**

A decentralized token wallet built on the Internet Computer Protocol (ICP), implemented in Rust. This project demonstrates how to manage token transfers, check balances, and log transactions securely using Rust smart contracts.

## **Features**
### **1. Send Tokens**

  * Allows users to send tokens to other wallet addresses securely.
  * Ensures sufficient balance checks before any transfer.
    
### **Receive Tokens**

  * Updates the wallet's balance automatically when tokens are received.

### **Balance Display**

  * Fetches and displays the current token balance for any wallet address.

### **Transaction History**

* Logs and provides access to a history of all wallet transactions.

### **Owner Initialization**

* The wallet is initialized with the caller as the owner and a default balance for testing.
  
## **Tech Stack**
- **Language**: Rust
- **Framework**: ICP SDK
- **Blockchain**: Internet Computer Protocol (ICP)
- **Testing Environment**: DFX Local Replica

## **Usage**
### **1. Check Balance**
- dfx canister call token_wallet_icp get_balance '(principal "<your-principal-id>")
### **2. Transfer Tokens**
- dfx canister call token_wallet_icp transfer '(principal "<recipient-principal-id>", <amount>)'
### **3. View All Balances**
- dfx canister call token_wallet_icp get_all_balances
### **4. View Transaction History**
- dfx canister call token_wallet_icp get_transactions

## **Testing**
### **Unit Tests**
To test the smart contract functionalities locally:

  1. Modify the Rust code in src/lib.rs to include test functions.
  2. Run the tests using: cargo test

## **Deployment to Testnet**
### **Switch to ICP Testnet**
Update the dfx.json file to include the testnet configuration:

Deploy
bash
Copy code
dfx deploy --network ic
Interact with Testnet Canister
Use similar commands as the local replica but with the --network ic flag:

bash
Copy code
dfx canister call token_wallet_icp get_balance --network ic '(principal "<your-principal-id>")'
Future Improvements
Token Metadata: Extend functionality to include token metadata like name, symbol, and decimals.
Security Enhancements: Add features like multi-signature transactions and access control.
Web Interface: Build a frontend for easier interaction with the wallet.
Contributing
Contributions are welcome! To contribute:

Fork the repository.
Create a feature branch (git checkout -b feature-name).
Commit your changes (git commit -m "Add new feature").
Push to the branch (git push origin feature-name).
Open a Pull Request.
License
This project is licensed under the MIT License.


