// src/components/Wallet.js
import React, { useState, useEffect } from 'react';
import { getBalance, transferTokens } from '../services/walletService';

function Wallet() {
  const [balance, setBalance] = useState(0);
  const [amount, setAmount] = useState(0);
  const [receiver, setReceiver] = useState('');

  const caller = "caller_principal_here"; // Replace with the actual caller's principal ID

  useEffect(() => {
    const fetchBalance = async () => {
      const fetchedBalance = await getBalance(caller);
      setBalance(fetchedBalance);
    };

    fetchBalance();
  }, [caller]);

  const handleTransfer = async () => {
    await transferTokens(amount, receiver);
    setBalance(balance - amount); // Update the balance after transfer
  };

  return (
    <div>
      <h1>Wallet Balance: {balance}</h1>
      <input
        type="number"
        value={amount}
        onChange={(e) => setAmount(Number(e.target.value))}
      />
      <input
        type="text"
        placeholder="Receiver's Principal ID"
        value={receiver}
        onChange={(e) => setReceiver(e.target.value)}
      />
      <button onClick={handleTransfer}>Transfer</button>
    </div>
  );
}

export default Wallet;