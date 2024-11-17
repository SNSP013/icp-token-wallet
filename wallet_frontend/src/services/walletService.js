// src/services/walletService.js
import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory as walletBackendIdl } from '../declarations/wallet_backend/wallet_backend.did';
import { _Principal } from '@dfinity/principal';

// Create the agent and actor to interact with the wallet_backend canister
const agent = new HttpAgent({ host: "http://localhost:8000", fetch: fetch });

const walletBackendActor = Actor.createActor(walletBackendIdl, {
  agent,
  canisterId: "your_wallet_backend_canister_id", // Replace with your actual wallet_backend canister ID
});

export const getBalance = async (caller) => {
  try {
    const balance = await walletBackendActor.get_balance(caller);
    return balance;
  } catch (error) {
    console.error("Error fetching balance:", error);
  }
};

export const transferTokens = async (amount, receiver) => {
  try {
    await walletBackendActor.transfer(amount, receiver);
  } catch (error) {
    console.error("Error transferring tokens:", error);
  }
};

export default walletBackendActor;